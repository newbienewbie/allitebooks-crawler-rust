extern crate regex;
extern crate select;

use select::document::Document ;
use select::predicate::Predicate;
use select::predicate::Class;
use select::predicate::Name;


#[derive(Debug)]
pub struct ArticleListItem{
    title:String,
    author:String,
    thumbnail_url:String,
    detail_url:String,
}


pub struct Parser{
    document: Document,
}

impl Parser {
    pub fn new(tendril : &str) -> Parser{
        Parser{
            document: Document::from(tendril),
        }
    }

    pub fn parse_current_page_info(&self) -> Option<(i32,i32)>{

        let page_info_element= self.document.find(Class("pagination").and(Name("div"))).next();
        
        let r=match page_info_element {
            Some(page_info) =>{
                let pages=page_info.find(Class("pages").and(Name("span"))).next();
                match pages {
                    Some(n) => Some(n.text()) ,
                    None => None
                }
            },
            None => None
        };


        match r {
            Some(page_info) =>{
                let reg =regex::Regex::new(r"^\s?(?P<currentpage>[\d]*)\s?.?\s?(?P<totalpages>[\d]*)\s?Pages").unwrap();
                let result= reg.captures(page_info.as_str()).unwrap();
                let current_page=result.get(1).map_or("", |m|m.as_str());
                let total_pages=result.get(2).map_or("", |m|m.as_str());
                let current_page=current_page.parse::<i32>().unwrap();
                let total_pages=total_pages.parse::<i32>().unwrap();
                Some( (current_page,total_pages))
            },
            None =>None
        }
    }


    pub fn parse_next_page_url(&self) -> Option<String>{
        match self.parse_current_page_info(){
            Some((current,total)) =>{
                let next=current+1;
                if next > total{
                    None
                }else{
                    let nextUrl=format!("/page/{}",next);
                    Some(nextUrl)
                }
            },
            None => None
        }
    }

    pub fn parse_article_list(&self) -> Option<Vec<ArticleListItem>>{
        let mut list:Vec<ArticleListItem>=Vec::new();
        for article_node in self.document.find(Name("article")){

            let img_selector=Name("div").and(Class("entry-thumbnail"))
                .descendant(Name("a"))
                .descendant(Name("img"));
            let img_node=article_node.find(img_selector).next().unwrap();
            let img_src= img_node.attr("src").unwrap();

            let header_selector=Name("div").and(Class("entry-body"))
                .child(Name("header").and(Class("entry-header")));
            let header_node=article_node.find(header_selector).next().unwrap();

            let title_selector=Name("h2").descendant(Name("a"));
            let title_node = header_node.find(title_selector).next().unwrap();

            let detail_url=title_node.attr("href").unwrap();
            let title=title_node.text();

            let author_selector=Name("div").and(Class("entry-meta")).descendant(Name("h5")).descendant(Name("a"));
            let author=header_node.find(author_selector).next().unwrap().text();

            let item=ArticleListItem{
                title:title,
                detail_url:detail_url.to_string(),
                author:author.trim().to_string(),
                thumbnail_url:img_src.to_string(),
            };
            list.push(item)
        }
        Some(list)
    }

}


