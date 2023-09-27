use html2md::parse_html;
use regex::Regex;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::chrono::NaiveDateTime;
use std::io::prelude::*;
use std::{fs, io};
use unidecode::unidecode;

static DATABASE_URL: &str = "mysql://root:password@127.0.0.1:3306/joomladb";
static DIR: &str = "./content";

// mos_sections

// | id               | int(11)             | NO   | PRI | NULL                | auto_increment |
// | title            | varchar(50)         | NO   |     |                     |                |
// | name             | varchar(255)        | NO   |     |                     |                |
// | image            | varchar(100)        | NO   |     |                     |                |
// | scope            | varchar(50)         | NO   | MUL |                     |                |
// | image_position   | varchar(10)         | NO   |     |                     |                |
// | description      | text                | NO   |     | NULL                |                |
// | published        | tinyint(1)          | NO   |     | 0                   |                |
// | checked_out      | int(11) unsigned    | NO   |     | 0                   |                |
// | checked_out_time | datetime            | NO   |     | 0000-00-00 00:00:00 |                |
// | ordering         | int(11)             | NO   |     | 0                   |                |
// | access           | tinyint(3) unsigned | NO   |     | 0                   |                |
// | count            | int(11)             | NO   |     | 0                   |                |
// | params           | text                | NO   |     | NULL                |                |

// mos_categories

// | id               | int(11)             | NO   | PRI | NULL                | auto_increment |
// | parent_id        | int(11)             | NO   |     | 0                   |                |
// | title            | varchar(50)         | NO   |     |                     |                |
// | name             | varchar(255)        | NO   |     |                     |                |
// | image            | varchar(100)        | NO   |     |                     |                |
// | section          | varchar(50)         | NO   | MUL |                     |                |
// | image_position   | varchar(10)         | NO   |     |                     |                |
// | description      | text                | NO   |     | NULL                |                |
// | published        | tinyint(1)          | NO   |     | 0                   |                |
// | checked_out      | int(11) unsigned    | NO   | MUL | 0                   |                |
// | checked_out_time | datetime            | NO   |     | 0000-00-00 00:00:00 |                |
// | editor           | varchar(50)         | YES  |     | NULL                |                |
// | ordering         | int(11)             | NO   |     | 0                   |                |
// | access           | tinyint(3) unsigned | NO   | MUL | 0                   |                |
// | count            | int(11)             | NO   |     | 0                   |                |
// | params           | text                | NO   |     | NULL                |                |

// mos_content

// | id               | int(11) unsigned | NO   | PRI | NULL                | auto_increment |
// | title            | varchar(100)     | NO   |     |                     |                |
// | title_alias      | varchar(100)     | NO   |     |                     |                |
// | introtext        | mediumtext       | NO   |     | NULL                |                |
// | fulltext         | mediumtext       | NO   |     | NULL                |                |
// | state            | tinyint(3)       | NO   | MUL | 0                   |                |
// | sectionid        | int(11) unsigned | NO   | MUL | 0                   |                |
// | mask             | int(11) unsigned | NO   | MUL | 0                   |                |
// | catid            | int(11) unsigned | NO   | MUL | 0                   |                |
// | created          | datetime         | NO   |     | 0000-00-00 00:00:00 |                |
// | created_by       | int(11) unsigned | NO   |     | 0                   |                |
// | created_by_alias | varchar(100)     | NO   |     |                     |                |
// | modified         | datetime         | NO   |     | 0000-00-00 00:00:00 |                |
// | modified_by      | int(11) unsigned | NO   |     | 0                   |                |
// | checked_out      | int(11) unsigned | NO   | MUL | 0                   |                |
// | checked_out_time | datetime         | NO   |     | 0000-00-00 00:00:00 |                |
// | publish_up       | datetime         | NO   |     | 0000-00-00 00:00:00 |                |
// | publish_down     | datetime         | NO   |     | 0000-00-00 00:00:00 |                |
// | images           | text             | NO   |     | NULL                |                |
// | urls             | text             | NO   |     | NULL                |                |
// | attribs          | text             | NO   |     | NULL                |                |
// | version          | int(11) unsigned | NO   |     | 1                   |                |
// | parentid         | int(11) unsigned | NO   |     | 0                   |                |
// | ordering         | int(11)          | NO   |     | 0                   |                |
// | metakey          | text             | NO   |     | NULL                |                |
// | metadesc         | text             | NO   |     | NULL                |                |
// | access           | int(11) unsigned | NO   | MUL | 0                   |                |
// | hits             | int(11) unsigned | NO   |     | 0                   |                |

// mos_users

// | id            | int(11)             | NO   | PRI | NULL                | auto_increment |
// | name          | varchar(50)         | NO   | MUL |                     |                |
// | username      | varchar(25)         | NO   |     |                     |                |
// | email         | varchar(100)        | NO   |     |                     |                |
// | password      | varchar(100)        | NO   |     |                     |                |
// | usertype      | varchar(25)         | NO   | MUL |                     |                |
// | block         | tinyint(4)          | NO   |     | 0                   |                |
// | sendEmail     | tinyint(4)          | YES  |     | 0                   |                |
// | gid           | tinyint(3) unsigned | NO   |     | 1                   |                |
// | registerDate  | datetime            | NO   |     | 0000-00-00 00:00:00 |                |
// | lastvisitDate | datetime            | NO   |     | 0000-00-00 00:00:00 |                |
// | activation    | varchar(100)        | NO   |     |                     |                |
// | params        | text                | NO   |     | NULL                |                |

#[derive(Debug, sqlx::FromRow)]
struct Section {
    id: i32,
    title: String,
    name: String,
    published: bool,
}

#[derive(Debug, sqlx::FromRow)]
struct Category {
    id: i32,
    title: String,
    name: String,
    published: bool,
}

#[derive(Debug, sqlx::FromRow)]
struct Content {
    id: u32,
    category: String,
    section: String,
    title: String,
    introtext: String,
    fulltext: String,
    author: String,
    author_alias: Option<String>,
    created: NaiveDateTime,
}

#[derive(Debug)]
struct HugoContent {
    category: String,
    title: String,
    author: String,
    date: NaiveDateTime,
    summary: String,
    content: String,
    tags: Vec<String>,
    draft: bool,
}

impl HugoContent {
    fn filename(&self) -> String {
        format!(
            "{DIR}/{}/{}-{}.md",
            sanitize(self.category.to_owned()),
            self.date.format("%Y%m%d"),
            sanitize(self.title.to_owned())
        )
    }

    fn write(&self) -> Result<(), io::Error> {
        if std::path::Path::new(&self.filename()).exists() {
            eprintln!("******* DUPLICATE: {:?}", self);
        }
        let mut file = fs::File::create(self.filename())?;
        file.write_all(self.content().as_bytes())?;
        Ok(())
    }

    fn content(&self) -> String {
        format!(
            r#"+++
Title = "{}"
Date = "{}"
Author = "{}"
Draft = {}
+++

{}

{}"#,
            self.title, self.date, self.author, self.draft, self.summary, self.content,
        )
    }
}

fn sanitize(filename: String) -> String {
    let re = Regex::new(r"[^A-Za-z0-9_-]").unwrap();
    let str = unidecode(&filename.trim().to_lowercase()).replace(" ", "_");
    re.replace_all(&str, "").to_string()
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    // Read Joomla data
    let categories =
        sqlx::query_as::<_, Category>("SELECT id,title,name,published from mos_categories")
            .fetch_all(&pool)
            .await?;
    let sections = sqlx::query_as::<_, Section>("SELECT id,title,name,published from mos_sections")
        .fetch_all(&pool)
        .await?;
    let contents = sqlx::query_as::<_, Content>(
        "SELECT c.id, cat.title AS category, s.title as section, c.title, c.introtext, c.fulltext, u.name AS author, c.created_by_alias AS author_alias, c.created
        FROM mos_content AS c
        INNER JOIN mos_categories AS cat ON cat.id = c.catid
        INNER JOIN mos_sections AS s ON c.sectionid = s.id
        INNER JOIN mos_users AS u ON u.id = c.created_by
        ",
    )
    .fetch_all(&pool)
    .await?;

    // Create Hugo content
    fs::create_dir_all(&DIR)?;
    //for section in &sections {
    //    let filename = format!("{}/{}", DIR, sanitize(section.title.to_owned()));
    //    fs::create_dir_all(filename)?;
    //}
    for category in &categories {
        let filename = format!("{}/{}", DIR, sanitize(category.title.to_owned()));
        fs::create_dir_all(filename)?;
    }
    for content in contents {
        println!("Content: {}", content.title);
        let content = HugoContent {
            category: content.category,
            title: content.title,
            author: content
                .author_alias
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| content.author),
            date: content.created,
            summary: parse_html(&content.introtext),
            content: parse_html(&content.fulltext),
            tags: vec![],
            draft: false,
        };
        content.write()?;
    }

    Ok(())
}
