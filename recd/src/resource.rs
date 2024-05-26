use crate::dependency::Dependency;
use crate::Error;
use crate::mahimahi;

use std::fs::File;
use std::path::Path;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::str;

use url;
use url::Url;

use protobuf::Message;

/// A trait for types with associated string name and value.
pub trait NameValue {
    /// Returns the object's name.
    fn name(&self) -> &[u8];

    /// Returns the object's value.
    fn value(&self) -> &[u8];
}

#[derive(Clone, Debug, PartialEq)]
pub struct Header(Vec<u8>, Vec<u8>);

impl Header {
    pub fn new(name: &[u8], value: &[u8]) -> Self {
        Self(name.to_vec(), value.to_vec())
    }
}

impl NameValue for Header {
    fn name(&self) -> &[u8] {
        &self.0
    }

    fn value(&self) -> &[u8] {
        &self.1
    }
}

pub struct Resources {
    transactions: HashMap<usize, Transaction>,
}

impl Resources {
    pub fn new(path: &Path, dependency: &Dependency) -> Result<Resources, Error> {
        let mut transactions = HashMap::new();
        let transaction_list = parse_transactions_path(path)?;

        for index in &dependency.net_activities() {
            let activity = dependency.activity(*index).unwrap();
            let url;
            
            // Exceptions
            if activity.url.starts_with("blob:") {
                // in yahoo.com, yy.com, ladbible.com
                // blob:https://www.ladbible.com/...
                url = url::Url::parse(activity.url.strip_prefix("blob:").unwrap()).unwrap();
            } else if activity.url.starts_with("about:") {
                // in aparat.com
                // about:srcdoc
                url = url::Url::parse("https://srcdoc").unwrap();
            } else if activity.url.starts_with("tmpURL/") {
                // in youtube.com and many webpages...
                // tmpURL/inlinedObject
                url = url::Url::parse("https://tmpURL/inlinedObject").unwrap();
            } else {
                url = url::Url::parse(&activity.url).unwrap();
            }

            match Resources::matched_transaction(&transaction_list, &url) {
                Some(transaction) => {
                    transactions.insert(*index, transaction.clone());
                }
                None => {
                    let domain = url.domain().unwrap_or("");
                    let mut path = String::from(url.path());
                    if let Some(query) = url.query() {
                        path.push('?');
                        path.push_str(query);
                    }
    
                    let pseudo_request = H3Request::new(domain, &path);
                    let pseudo_response = H3Response::new(404, Vec::new(), &[]);
                    let pseudo_transaction = Transaction::new(pseudo_request, pseudo_response);
                    transactions.insert(*index, pseudo_transaction);
                }
            }
        }
        Ok(Resources { transactions })
    }

    // Same with mahimahi
    fn matched_transaction<'a>(transactions: &'a Vec<Transaction>, request_url: &Url) -> Option<&'a Transaction> {
        let domain = match request_url.domain() {
            Some(v) => v,
            None => return None
        };
        let path = request_url.path();
        let query = request_url.query();

        let mut maximum_score = None;
        let mut matched_transaction = None;
        for transaction in transactions {
            if let Some(score) = Resources::matched_line(transaction, domain, path, query) {
                if let Some(v) = maximum_score {
                    if score > v {
                        maximum_score = Some(v);
                        matched_transaction = Some(transaction);
                    }
                } else {
                    maximum_score = Some(score);
                    matched_transaction = Some(transaction);
                }
            }
        }

        // println!("");
        // if let Some(v) = maximum_score {
        //     println!("{}\nSaved: https://{}{}\nQuery: {}", v, str::from_utf8(matched_transaction.unwrap().authority().unwrap()).unwrap(), str::from_utf8(matched_transaction.unwrap().path().unwrap()).unwrap(), request_url);
        // } else {
        //     println!("Not Found\nQuery: {}", request_url);
        // }
        
        matched_transaction
    }

    fn matched_line(transaction: &Transaction, domain: &str, path: &str, query: Option<&str>) -> Option<usize> {
        let saved_authority = str::from_utf8(transaction.authority().expect("the transaction does not have a `:authority` header")).unwrap();
        let saved_path_and_query = str::from_utf8(transaction.path().expect("the transaction does not have a `:path` header")).unwrap();
        let (saved_path, saved_query) = Resources::split_query(saved_path_and_query);
        
        if saved_authority != domain {
            return None;
        }

        if saved_path != path {
            return None;
        }

        let saved_path_and_query = if let Some(v) = saved_query {
            format!("{}?{}", saved_path, v)
        } else {
            saved_path
        };

        let path_and_query = if let Some(v) = query {
            format!("{}?{}", path, v)
        } else {
            path.to_string()
        };

        // println!("Saved: {}{} ({})", saved_authority, saved_path_and_query, saved_path_and_query.as_bytes().len());
        // println!("Query: {}{} ({})", domain, path_and_query, path_and_query.as_bytes().len());

        let saved = format!("{}{}", saved_authority, saved_path_and_query);
        let saved = saved.as_bytes();
        let request = format!("{}{}", domain, path_and_query);
        let request = request.as_bytes();
        let max_match = cmp::min(saved.len(), request.len());
        for i in 0..max_match {
            if saved[i] != request[i] {
                // println!("{}", i);
                return Some(i);
            }
        }

        // println!("{}", max_match);
        Some(max_match)
    }

    fn split_query(path_and_query: &str) -> (String, Option<String>) {
        let split = path_and_query.split("?").collect::<Vec<&str>>();
        if split.len() == 0 {
            panic!("split failed.");
        } else if split.len() == 1 {
            (split[0].to_string(), None)
        } else {
            (split[0].to_string(), Some(split[1].to_string()))
        }
    }

    pub fn get(&self, activity_id: &usize) -> Option<&Transaction> {
        self.transactions.get(&activity_id)
    }

    pub fn find_ids_by_request(&self, request: &H3Request) -> Vec<usize> {
        let mut ids = Vec::new();

        for (id, trans) in &self.transactions {
            if trans.check_request(&request) == true {
                ids.push(*id);
            }
        }

        if ids.len() == 0 {
            panic!("There is no such ID, header={}", request);
        }
        ids
    }
}

/// Generate the resources from traces
pub fn parse_transactions_path(path: &Path) -> Result<Vec<Transaction>, Error> {
    let mut transaction_list = Vec::new();
    for trace in path.read_dir().expect("read_dir failed.") {
        if let Ok(trace) = trace {
            if matches!(trace.path().extension(), Some(v) if v == "json") {
                continue;
            }
            let mut file = File::open(trace.path()).expect("failed on opening file.");
            let content = mahimahi::RequestResponse::parse_from_reader(&mut file).unwrap();

            // Generate request example
            // :method: GET (or POST)
            // :scheme: HTTP/3
            // :authority: www.youtube.com
            // :path: /s/player/e3cd195e/player_ias.vflset/en_US/embed.js
            // :user-agent: quiche
            let request = content.get_request();

            // request_first_line example
            // GET /s/player/e3cd195e/player_ias.vflset/en_US/embed.js HTTP/1.1
            let request_first_line =
                String::from_utf8_lossy(request.get_first_line()).into_owned();
            let headers = request.get_header();
            let mut host = None;
            for header in headers {
                let key = String::from_utf8_lossy(header.get_key()).into_owned();
                let value = String::from_utf8_lossy(header.get_value()).into_owned();
                if key == "Host" {
                    host = Some(value);
                }
            }
            let request_items = request_first_line.split(' ').collect::<Vec<_>>();

            let mut header_list = Vec::new();
            header_list.push(Header::new(
                b":method",
                request_items[0].as_bytes(),
            ));
            header_list.push(Header::new(b":scheme", b"HTTP/3"));
            if let Some(v) = host {
                header_list.push(Header::new(b":authority", &v.as_bytes()));
            } else {
                panic!("there is no host!");
            }
            header_list.push(Header::new(
                b":path",
                request_items[1].as_bytes(),
            ));
            header_list.push(Header::new(b":user-agent", b"quiche"));
            let request = H3Request { header_list };

            // Generate response
            let response = content.get_response();

            // response_first_line example
            // HTTP/1.1 200 OK
            let response_first_line =
                String::from_utf8_lossy(response.get_first_line()).into_owned();
            let response_items = response_first_line.split(' ').collect::<Vec<_>>();
            let status = response_items[1].parse::<usize>().unwrap();

            let mut header_list = Vec::new();
            for header in headers {
                let key = header.get_key();
                let value = header.get_value();
                header_list.push(Header::new(key, value));
            }

            let body = Vec::from(response.get_body());
            let response = H3Response { status, header_list, body };

            let transaction = Transaction {
                request,
                response,
            };
            transaction_list.push(transaction);
        }
    }

    Ok(transaction_list)
}

#[derive(Clone)]
pub struct Transaction {
    request: H3Request,
    response: H3Response,
}

impl Transaction {
    pub fn new(request: H3Request, response: H3Response) -> Transaction {
        Transaction { request, response }
    }

    /// Returns the request header's authority
    pub fn authority(&self) -> Option<&[u8]> {
        for header in &self.request.header_list {
            if header.name() == b":authority" {
                return Some(header.value());
            }
        }
        return None;
    }

    /// Returns the request header's path
    pub fn path(&self) -> Option<&[u8]> {
        for header in &self.request.header_list {
            if header.name() == b":path" {
                return Some(header.value());
            }
        }
        return None;
    }

    /// Returns the request header's scheme
    pub fn scheme(&self) -> Option<&[u8]> {
        for header in &self.request.header_list {
            if header.name() == b":scheme" {
                return Some(header.value());
            }
        }
        return None;
    }

    /// Returns the request header's path
    pub fn method(&self) -> Option<&[u8]> {
        for header in &self.request.header_list {
            if header.name() == b":method" {
                return Some(header.value());
            }
        }
        return None;
    }

    pub fn request(&self) -> &H3Request {
        &self.request
    }

    pub fn response(&self) -> &H3Response {
        &self.response
    }

    pub fn check_request(&self, request: &H3Request) -> bool {
        let mut src_headers = HashMap::new();
        for hdr in &request.header_list {
            let name = hdr.name();
            let value = hdr.value();
            src_headers.insert(name, value);
        }

        let mut dst_headers = HashMap::new();
        for hdr in &self.request.header_list {
            let name = hdr.name();
            let value = hdr.value();
            dst_headers.insert(name, value);
        }

        let mut same = true;
        for (src_hdr_name, src_hdr_value) in src_headers {
            match dst_headers.remove(src_hdr_name) {
                Some(v) => {
                    if src_hdr_value != v {
                        same = false;
                        break;
                    }
                }
                None => {
                    same = false;
                    break;
                }
            }
        }

        if dst_headers.len() != 0 {
            same = false;
        }

        same
    }
}

#[derive(Clone)]
pub struct H3Request {
    header_list: Vec<Header>,
}

impl H3Request {
    pub fn new(domain: &str, path: &str) -> H3Request {
        let mut header_list = Vec::new();

        header_list.push(Header::new(b":method", b"GET"));
        header_list.push(Header::new(b":scheme", b"HTTP/3"));
        header_list.push(Header::new(b":authority", domain.as_bytes()));
        header_list.push(Header::new(b":path", path.as_bytes()));
        header_list.push(Header::new(b":user-agent", b"quiche"));

        H3Request { header_list }
    }

    pub fn header_list(&self) -> &Vec<Header> {
        &self.header_list
    }
}

impl From<Vec<Header>> for H3Request {
    fn from(list: Vec<Header>) -> Self {
        H3Request { header_list: list }
    }
}

impl fmt::Display for H3Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let headers: Vec<String> = self
            .header_list
            .iter()
            .map(|s| {
                format!(
                    "{}: {}",
                    str::from_utf8(s.name()).unwrap(),
                    str::from_utf8(s.value()).unwrap()
                )
            })
            .collect();
        write!(f, "{}", headers.join("\n"))
    }
}

#[derive(Clone)]
pub struct H3Response {
    status: usize,
    header_list: Vec<Header>,
    body: Vec<u8>,
}

impl H3Response {
    pub fn new(status: usize, header_list: Vec<Header>, body: &[u8]) -> H3Response {
        H3Response {
            status,
            header_list,
            body: body.to_vec(),
        }
    }

    pub fn status(&self) -> usize {
        self.status
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
}

impl fmt::Display for H3Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let headers: Vec<String> = self
            .header_list
            .iter()
            .map(|s| {
                format!(
                    "{}: {}",
                    str::from_utf8(s.name()).unwrap(),
                    str::from_utf8(s.value()).unwrap()
                )
            })
            .collect();
        write!(f, "{} ({})\n{}", self.status, self.body.len(), headers.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_test() {
        let path = "/home/sunj/lrs/traces-1/bing.com/run-0/";
        let ts = parse_transactions_path(path.as_ref()).unwrap();

        for t in ts {
            println!("{}\nstatus: {} size: {}", t.request, t.response.status, t.response.body.len());
        }
    }
}