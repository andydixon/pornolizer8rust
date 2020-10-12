use reqwest::blocking::Response;

pub struct url_content {
    pub(crate) response: Response,
    pub(crate) body: String
}
pub(crate) mod webpage_handler {
    use error_chain::error_chain;
    use std::io::Read;
    use reqwest;
    use reqwest::blocking::Response;
    use crate::webpage_handler::url_content;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    pub fn parse_url(url: String) ->url_content {
        let mut resp = reqwest::blocking::get(&url).unwrap();
        let mut body = String::new();
        resp.read_to_string(&mut body).unwrap();

        let content = url_content {
            response: resp,
            body: body
        };
        return content
    }

}
