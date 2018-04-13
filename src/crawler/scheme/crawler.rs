
use ::reqwest::Client;
use ::reqwest::Url;
use ::reqwest::header::UserAgent;
use ::reqwest::header::Headers;
use ::std::time::Duration;
use super::parser::Parser;

pub struct Crawler {
    pub host: String,
    pub client: Client,
}

impl Crawler{
    pub fn new(host:String) -> Crawler{
        let client=Client::builder()
            .timeout(Duration::from_secs(6*60*60))  // 6 hours
            .build()
            .unwrap();
        Crawler{
            host:host,
            client:client,
        }
    }

    pub fn crawl_one_page(&self,url: &str) -> Result<String, ::reqwest::Error> {
        let mut headers=Headers::new();
        headers.set(UserAgent::new("Mozilla/5.0 (Windows NT 10.0; WOW64; rv:53.0) Gecko/20100101 Firefox/53.0"));
        let x=self.client.get(url)
            .headers(headers)
            .send()?
            .text()?;
        Ok(x)
    }

    pub fn crawl_page_recursive(&self, url:&str , retry: i32) -> Result<String,(i32)> {
        let err=Err(0); 
        if retry <= 0{ 
            return err; 
        }
        println!("正在抓取 : {} \t...",url);
        let result=self.crawl_one_page(url);

        match result {
            Ok(text) =>{
                let parser=Parser::new(&text);
                let articles=parser.parse_article_list().unwrap();
                // todo : persisit articles
                println!("捕获到文章项目：{:?}", articles);
                let next_url=parser.parse_next_page_url();
                next_url.ok_or(0)
            },
            Err(x) => {
                let retry=retry-1;
                println!("{:?}", x);
                println!("抓取失败：{}。剩余次数：{}", url,retry);
                self.crawl_page_recursive(url, retry)
            }
        }

    }

    fn generate_next_url(&self,next:&str) -> Result<Url,::reqwest::UrlError> {
        Url::parse( &self.host )?
            .join(next)
    }

    pub fn crawl(&self , seed : &str){
        let next=self.crawl_page_recursive(seed, 3);
        if let Ok(next) = next {
            println!("捕获到下一页: 《{}》，开始抓取", next);
            match self.generate_next_url(&next) {
                Ok(next) => self.crawl(next.as_str()) ,
                Err(msg)  => println!("解析下一页地址错误，抓取结束 ... \r\ncurrent host: {} \r\n next url {}", self.host,msg)
            }
        }else{
            println!("未抓取到下一页，抓取结束");
        }
    }
}