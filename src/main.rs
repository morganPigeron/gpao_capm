use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

/*
 * Achat et commande
 *
 * Un article à un délai de livraison.
 * Un article provient d'un point A et arrive à un point B
 *
 */

#[derive(Eq, PartialEq)]
enum PointCondition<'a> {
    Empty,
    ArticlesCount(HashMap<&'a Article<'a>, usize>),
}

impl Hash for PointCondition<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            PointCondition::Empty => {
                state.write_u8(0);
            }
            PointCondition::ArticlesCount(map) => {
                state.write_u8(1);
                for (k, v) in map {
                    k.hash(state);
                    v.hash(state);
                }
            }
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Point<'a> {
    name: String,
    condition: PointCondition<'a>,
}

impl Point<'_> {
    fn new(name: String) -> Point<'static> {
        Point {
            name,
            condition: PointCondition::Empty,
        }
    }

    fn add_condition<'a>(&'a self, article: &'a Article, quantity: usize) -> Point {
        let mut map = HashMap::new();
        map.insert(article, quantity);
        Point {
            name: self.name.clone(),
            condition: PointCondition::ArticlesCount(map),
        }
    }
}

impl fmt::Display for Point<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{}", self.name));
        write!(f, "{}", result)
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Route<'a> {
    point_a: Point<'a>,
    point_b: Point<'a>,
    time_to_completion: Option<Duration>,
    start_date: Option<DateTime<Utc>>,
}

impl Route<'_> {
    fn new() -> Route<'static> {
        Route {
            point_a: Point::new("".to_string()),
            point_b: Point::new("".to_string()),
            time_to_completion: None,
            start_date: None,
        }
    }
}

impl fmt::Display for Route<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("route: {} -> {}", &self.point_a, &self.point_b));
        if let (Some(time_to_completion), Some(start_date)) =
            (&self.time_to_completion, &self.start_date)
        {
            let end_date: DateTime<Utc> = start_date
                .checked_add_signed(*time_to_completion)
                .expect("cannot convert date");
            result.push_str(&format!(
                ", start date: {} -> end date: {}",
                start_date, end_date
            ));
        } else if let Some(time_to_completion) = &self.time_to_completion {
            let end_date: DateTime<Utc> = Utc::now()
                .checked_add_signed(*time_to_completion)
                .expect("cannot convert date");
            result.push_str(&format!(", end date from now: {}", end_date));
        } else if let Some(start_date) = &self.start_date {
            result.push_str(&format!(", Available by: {}", start_date));
        }
        write!(f, "{}", result)
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Article<'a> {
    name: String,
    route: Option<Route<'a>>,
}

impl Article<'_> {
    fn new(name: String) -> Article<'static> {
        Article {
            name: name.to_string(),
            route: None,
        }
    }
}

impl fmt::Display for Article<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("Article: {}", self.name));
        if let Some(route) = &self.route {
            result.push_str(&format!(", {}", route));
        }
        write!(f, "{}", result)
    }
}

struct Storage<'a> {
    name: String,
    articles: HashMap<&'a Article<'a>, usize>,
}

impl<'a> Storage<'a> {
    fn new(name: String) -> Storage<'static> {
        Storage {
            name,
            articles: HashMap::new(),
        }
    }

    fn add_articles(&mut self, article: &'a Article, quantity: usize) {
        // need to add an article if it doesnt exist or update quantity
        self.articles.insert(article, quantity);
    }
}

impl fmt::Display for Storage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("Store |{}", self.name));
        result.push_str(&format!(", total article refs: {}\n", self.articles.len()));

        for (article, qty) in &self.articles {
            result.push_str(&format!("      |-> Article {} x {}\n", article.name, qty));
        }

        write!(f, "{}", result)
    }
}

fn main() {
    println!("GPAO ... ");
    println!("Create a list of articles");

    let mut a = Article::new("A".to_string());
    a.route = Some(Route {
        point_a: Point::new("p1".to_string()),
        point_b: Point::new("p2".to_string()),
        time_to_completion: Some(Duration::days(5)),
        start_date: Some(Utc::now()),
    });

    let mut b = Article::new("B".to_string());
    b.route = Some(Route {
        point_a: Point::new("p2".to_string()),
        point_b: Point::new("p3".to_string()),
        time_to_completion: Some(Duration::days(4)),
        start_date: None,
    });

    let mut c = Article::new("C".to_string());
    c.route = Some(Route {
        point_a: Point::new("p4".to_string()),
        point_b: Point::new("p5".to_string()),
        time_to_completion: None,
        start_date: Some(Utc::now()),
    });

    // create an item who need two other items by composition
    let mut d = Article::new("D".to_string());
    let point_6 = Point::new("p6".to_string());
    d.route = Some(Route {
        point_a: point_6.add_condition(&a, 1),
        point_b: Point::new("p7".to_string()),
        time_to_completion: None,
        start_date: Some(Utc::now()),
    });

    // create a set of item
    // with different routes, delay and start date
    let articles = vec![&a, &b, &c, &d];

    for article in articles {
        println!("{}", article);
    }

    println!("Create storages that store articles");
    // create two storages
    // one with item A and another with item B
    let mut store_a = Storage::new("Store A".to_string());
    store_a.add_articles(&a, 10);

    let mut store_b = Storage::new("Store B".to_string());
    store_b.add_articles(&b, 5);

    let stores = vec![store_a, store_b];

    for store in stores {
        println!("{}", store);
    }

    // be able to ask when item will be available
}
