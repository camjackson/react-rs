pub trait Component {
    fn render(&self) -> String;
}

pub struct Div {
    pub children: Vec<Box<Component>>,
    pub class_name: &'static str,
}

pub struct Img {
    pub src: &'static str,
}

impl Component for Div {
    fn render(&self) -> String {
        render_dom_component("div", self.props(), self.render_children())
    }
}

impl Div {
    fn props(&self) -> String {
        let mut props = String::new();
        if self.class_name.len() > 0 {
            props = props + &format!(" class=\"{}\"", self.class_name)
        }
        props
    }

    fn render_children(&self) -> String {
        self.children.iter().map(|child| {
            child.render()
        }).collect()
    }
}

impl Component for Img {
    fn render(&self) -> String {
        render_void_dom_component("img", self.props())
    }
}

impl Img {
    fn props(&self) -> String {
        let mut props = String::new();
        if self.src.len() > 0 {
            props = props + &format!(" src=\"{}\"", self.src)
        }
        props
    }
}

fn render_dom_component(name: &'static str, props: String, children: String) -> String {
    format!("<{}{}>{}</{}>", name, props, children, name)
}

fn render_void_dom_component(name: &'static str, props: String) -> String {
    format!("<{}{}/>", name, props)
}

#[test]
fn it_renders_an_empty_div() {
    let div = Div { class_name: "", children: vec!() };
    assert_eq!(div.render(), "<div></div>");
}

#[test]
fn it_renders_an_img() {
    let img = Img { src: "" };
    assert_eq!(img.render(), "<img/>");
}

#[test]
fn it_renders_a_div_with_a_class() {
    let div = Div { class_name: "container", children: vec!() };
    assert_eq!(div.render(), "<div class=\"container\"></div>");
}

#[test]
fn it_renders_an_img_with_a_src() {
    let img = Img { src: "pic.jpg" };
    assert_eq!(img.render(), "<img src=\"pic.jpg\"/>");
}

#[test]
fn it_renders_a_div_with_a_child_div() {
    let nested_div = Box::new(Div { class_name: "", children: vec!() });
    let div = Div { class_name: "", children: vec!(nested_div) };
    assert_eq!(div.render(), "<div><div></div></div>");
}

#[test]
fn it_renders_a_complicated_thing() {
    let div = Div {class_name: "container", children: vec!(
        Box::new(Div {class_name: "row", children: vec!(
            Box::new(Div {class_name: "col-md-4", children: vec!()}),
            Box::new(Div {class_name: "col-md-4 col-md-offset-4", children: vec!(
                Box::new(Img {src: "https://example.com/img.jpg"}))
            })
        )}),
        Box::new(Div {class_name: "row", children: vec!(
            Box::new(Div {class_name: "col-md-6", children: vec!()}),
            Box::new(Div {class_name: "col-md-6", children: vec!()})
        )}),
    )};

    let dom = div.render();

    assert_eq!(dom, concat!(
        "<div class=\"container\">",
            "<div class=\"row\">",
                "<div class=\"col-md-4\">",
                "</div>",
                "<div class=\"col-md-4 col-md-offset-4\">",
                    "<img src=\"https://example.com/img.jpg\"/>",
                "</div>",
            "</div>",
            "<div class=\"row\">",
                "<div class=\"col-md-6\">",
                "</div>",
                "<div class=\"col-md-6\">",
                "</div>",
            "</div>",
        "</div>"
    ));
}

