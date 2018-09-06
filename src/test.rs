///
/// Foo bar fizz buzz
macro_rules! pub_struct {
  (
  $( # [ $struct_attributes:meta ] )* $name:ident {
    $( $( # [ $field_attributes:meta ] )* $field:ident : $fieldtype:ty , )+
  }
  ) => {
    $(#[$struct_attributes])* pub struct $name {
      $($( # [ $field_attributes ] )* pub $field : $fieldtype , )+
    }
  }
}



/////

pub_struct!{#[derive(Debug)] Example {
    x: f32,
}}

fn foo(x: Example) -> bool {

}