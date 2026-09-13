use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Article {
    id: u32,
    nom: String,
    prix: u32,
    stock: bool
}

impl Article {
    fn new(id: u32, nom: String, prix: u32, stock: bool) -> Self {
        Article { id, nom, prix, stock }
    }
}

trait ArticleRepository {
    fn save(&mut self, article: Article) -> ();
    fn find(&self, id: u32) -> Result<Article, String>;
    fn all(&self) -> impl Iterator<Item = &Article>;
}

#[derive(Default)]
struct VecCatalog {
    articles: Vec<Article>,
}

impl VecCatalog {
    fn new() -> Self {
        Self::default()
    }
}

impl ArticleRepository for VecCatalog {

    fn save(&mut self, article: Article) -> () {
        self.articles.push(article);
    }

    fn find(&self, id: u32) -> Result<Article, String> {

        self.articles.iter()
            .find(|article| article.id == id)
            .cloned()
            .ok_or_else(|| format!("Recherche id {}", id))
    }

    fn all(&self) -> impl Iterator<Item = &Article> {
        self.articles.iter()
    }

}

#[derive(Default)]
struct MapCatalog {
    articles: HashMap<u32, Article>,
}

impl MapCatalog {
    fn new() -> Self {
        Self::default()
    }
}

impl ArticleRepository for MapCatalog {
    fn save(&mut self, article: Article) -> () {
        self.articles.insert(article.id ,article);
    }

    fn find(&self, id: u32) -> Result<Article, String> {
        self.articles.get(&id).cloned().ok_or_else(|| format!("Recherche id {}", id))
    }

    fn all(&self) -> impl Iterator<Item = &Article> {
        self.articles.values()
    }

}

fn valeur_du_stock(repo: &impl ArticleRepository) -> u32 {
    repo.all().filter(|a| a.stock).map(|a| a.prix).sum()
}

fn prix_en_euros(centimes: u32) -> f64 {
    centimes as f64 / 100.0
}

fn main() {

    // Pour le Vec
    let mut stock_livre = VecCatalog::new();

    let le_monde = Article::new(01, "Le Monde".into(), 1999, true);
    let mosaiquelemag = Article::new(02, "Mosaïque".into(), 2499, true);
    let camino = Article::new(03, "Camino".into(), 1599, true);
    let baguarre_studio = Article::new(04, "baguarre.studio".into(), 999, false);

    stock_livre.save(le_monde);
    stock_livre.save(mosaiquelemag);
    stock_livre.save(camino);
    stock_livre.save(baguarre_studio);

    println!("Valeur du stock (Vec) : {} euros",  prix_en_euros(valeur_du_stock(&stock_livre)) );

    let requete_livre: [u32; 2] = [02, 99];

    for rl in requete_livre {
        match stock_livre.find(rl){
            Ok(a) => println!("Recherche id {} : trouve -> {}", a.id, a.nom),
            Err(e) => eprintln!("{} ,aucun article", e),
        };
    }

    println!("____________\n");

    //Pour le HashMap
    let mut stock_garage = MapCatalog::new();

    let roue = Article::new(10, "Roue".into(), 9499, true);
    let porte = Article::new(11, "Porte".into(), 5499, true);
    let moteur = Article::new(12, "Moteur".into(), 5900, true);
    let huile_moteur = Article::new(13, "Huile Moteur".into(), 1900, false);

    stock_garage.save(roue);
    stock_garage.save(porte);
    stock_garage.save(moteur);
    stock_garage.save(huile_moteur);

    println!("Valeur du stock (HashMap) : {} euros",  prix_en_euros(valeur_du_stock(&stock_garage)) );

    let requete_piece_garage: [u32; 2] = [12, 99];

    for rpg in requete_piece_garage {
        match stock_garage.find(rpg){
            Ok(a) => println!("Recherche id {} : trouve -> {}", a.id, a.nom),
            Err(e) => eprintln!("{} ,aucun article", e),
        };
    }
}
