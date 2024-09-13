struct Config {
    name: String,
    base_path: String,
    domain: String,
}

struct Post {
    path: String,
    title: String,
    body: String,
}

struct Website {
    config: Config,
    posts: Vec<Post>,
}
