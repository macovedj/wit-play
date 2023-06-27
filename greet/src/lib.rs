struct Component;

impl bindings::exports::component::greet::doer::Doer for Component {
// impl bindings::Component for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
    fn hello_name(name: String) -> String {
      let mut greeting = String::from("Hello ");
      greeting.push_str(&name);
      greeting
  }
}

impl bindings::exports::component::greet::additional::Additional for Component {
  // impl bindings::Component for Component {
      /// Say hello!
      fn additional_world() -> String {
          "Hello, World!".to_string()
      }
      fn additional_name(name: String) -> String {
        let mut greeting = String::from("Hello ");
        greeting.push_str(&name);
        greeting
    }
  }
bindings::export!(Component);
