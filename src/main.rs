#[allow(unused)]
#[derive(Clone)]
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

#[allow(unused)]
trait ArticleRepository {
    fn save(&mut self, article: Article) -> ();
    fn find(&self, id: u32) -> Option<Article>;
    fn all(&self) -> &[Article];
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

    fn find(&self, id: u32) -> Option<Article> {
        self.articles.iter().find(|article| article.id == id).cloned()
    }

    fn all(&self) -> &[Article] {
        &self.articles
    }

}

fn valeur_du_stock(repo: &impl ArticleRepository) -> u32 {
    repo.all().iter().filter(|a| a.stock).map(|a| a.prix).sum()
}

fn main() {

    let le_monde = Article::new(01, "Le Monde".into(), 1999, true);
    let mosaiquelemag = Article::new(02, "Mosaïque".into(), 2499, true);
    let camino = Article::new(03, "Camino".into(), 1599, true);
    let baguarre_studio = Article::new(04, "baguarre.studio".into(), 999, false);

    let mut stock_livre = VecCatalog::new();

    stock_livre.save(le_monde);
    stock_livre.save(mosaiquelemag);
    stock_livre.save(camino);
    stock_livre.save(baguarre_studio);

    println!("Valeur du stock (Vec) : {} centimes", valeur_du_stock(&stock_livre));

}
