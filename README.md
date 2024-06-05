# A Simple project on creating a Custom Derive Macro

In this simple project, we created our very own custom derive macro.
We created a crate named hello_proc_macro that prints the name of the type a trait is implemented on.

Note: This project expects you to have basic knowledge of rust. It will not go into the details of language features that are irrelevant to it. However, no prerequisite knowledge of creating or implementing custom derive macros is required to follow along.

Let's dive in!

## What are we creating?

We'll create a crate named hello_proc_macro that defines a trait named HelloProcMacro with one
associated function named hello_proc_macro .

The default implementation will print, for example; Hello, the name of your type is TypeName where TypeName is the name of the type on which this trait has been defined.

In other words, we’ll write a crate that enables another programmer to write code like;

```rust
use hello_proc_macro::HelloProcMacro;
use hello_proc_macro_derive::HelloProcMacro;

#[derive(HelloProcMacro)]
struct Mountain;

fn main() {
Mountain::hello_proc_macro();
}
```

This code will print "Hello, the name of your type is Mountain.", when we're done.

We'll start by creating our hello_proc_macro crate using the command below

```shell
cargo new hello_proc_macro --lib
```

This will create a new rust project with a lib.rs file for your hello_proc_macro crate.

Next we're going to define our trait which we will later call in our Derive Macro crate.

```rust
pub trait HelloProcMacro {
    fn hello_proc_macro ();
}
```

In the above code, we defined a trait named HelloProcMacro which has an empty associated function also named hello_proc_macro

## Why is our derive macro important?

This is where we'll talk about why using derive is important. Let's try to implement our trait for a type without using the derive macro.

```rust
use hello_proc_macro::HelloProcMacro;

struct Mountain;

impl HelloProcMacro for Mountain {
fn hello_proc_macro() {
println!("Hello, the name of your type is Mountain");
}
}

fn main() {
Mountain::hello_proc_macro();
}
```

Now the reason we won't be implementing our trait using the above format is because if we were to create another type (which is not named Mountain) that need the same trait, we would have to do another implemention for our new type. Let's say we needed multiple types with that same trait, it means we would have to repeatedly implement that trait for our different types.

Additionally, we can’t yet provide the hello_proc_macro function with default implementation
that will print the name of the type the trait is implemented on: Rust doesn’t have reflection
capabilities, so it can’t look up the type’s name at runtime. We need a macro to generate
code at compile time and this is where our derive macro comes in!

Let's define our procedural macro

## Creating our Procedural Macro

Within our existing project directory, we'll create a new project named hello_proc_macro_derive

### Naming convention

At the time of this writing, procedural macros need to be in their own crate. Eventually, this restriction might be lifted. The
convention for structuring crates and macro crates is as follows: for a crate named foo , a custom derive procedural macro crate is called foo_derive.

This means that for a crate named hello_proc_macro, we'll create a procedural macro crate named hello_proc_macro_derive

```shell
cargo new hello_proc_macro_derive --lib
```

This above command will create a crate named hello_proc_macro_derive with an src folder that contains a single lib.rs file and a
Cargo.toml

If we change the trait definition in hello_proc_macro, we’ll have to change the implementation of the procedural macro in hello_proc_macro_derive as well. The two crates will need to be published separately, and programmers using these crates will need to add both as dependencies and bring them both into scope. We could instead have the hello_proc_macro crate use hello_proc_macro_derive as a dependency and re-export the procedural macro code. However, the way we’ve structured the project makes it possible for programmers to use hello_proc_macro even if they don’t want the derive functionality. We need to declare the hello_proc_macro_derive crate as a procedural macro crate.

### Adding dependencies

We’ll also need functionality from the syn and quote crates, as you’ll see in a moment, so we need to
add them as dependencies. Add the following to the Cargo.toml file for hello_proc_macro_derive.

```rust
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

Now let's start defining our procedural macro in the lib.rs file of our hello_proc_macro_derive crate.

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloProcMacro)]
pub fn hello_proc_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap;

    // Build the trait implementation
    impl_hello_proc_macro(&ast)
}
```

In the above code, the hello_proc_macro_derive function is responsible for parsing the TokenStream, while the impl_hello_proc_macro function which we called, is responsible for transforming the syntax tree: this makes writing a procedural macro more convenient. The code in the outer function ( hello_proc_macro_derive in this case) will be the same for almost every procedural macro crate you see or create.

Note: You might have noticed that we’re calling unwrap to cause the hello_proc_macro_derive function to panic if the call to the syn::parse function fails here. It’s necessary for our procedural macro to panic on errors because proc_macro_derive functions must return TokenStream rather than Result to conform to the procedural macro API. We’ve simplified this example by using unwrap; in production code, you should provide more specific error messages about what went wrong by using panic! or expect.

Also, note that the output for our derive macro is also a TokenStream. The returned TokenStream is added to the code that our crate users write, so when they compile their crate, they’ll get the extra functionality that we provide in the modified TokenStream.

We’ve introduced three new crates:

### proc_macro

```rust
use proc_macro::TokenStream;
```

The proc_macro crate comes with Rust, so we didn’t need to add that to the dependencies in Cargo.toml. The
proc_macro crate is the compiler’s API that allows us to read and manipulate Rust code
from our code.

### syn

```rust
use syn;
```

The syn crate parses Rust code from a string into a data structure that we can perform
operations on.

### quote

```rust
use quote::quote;
```

The quote crate turns syn data structures back into Rust code. These crates make it much simpler to parse any sort of Rust code we might want to handle: writing a full parser for Rust code is no simple task.

The hello_proc_macro_derive function will be called when a user of our library specifies #[derive(HelloProcMacro)] on a type. This is possible because we’ve annotated the hello_proc_macro_derive function here with proc_macro_derive and specified the name
HelloProcMacro, which matches our trait name; this is the convention most procedural macros follow.

The hello_proc_macro_derive function first converts the input from a TokenStream to a data
structure that we can then interpret and perform operations on. This is where syn comes
into play. The parse function in syn takes a TokenStream and returns a DeriveInput
struct representing the parsed Rust code.

### DeriveInput struct

Let's see the relevant parts of the DeriveInput struct

```rust
DeriveInput {
// --snip--
ident: Ident {
ident: "Mountain",
span: #0 bytes(95..103)
},
data: Struct(
DataStruct {
struct_token: Struct,
fields: Unit,
semi_token: Some(
Semi
)
}
)
}
```

The fields of this struct show that the Rust code we’ve parsed is a unit struct with the ident (identifier, meaning the name) of Mountain. There are more fields on this struct for describing all sorts of Rust code; check the syn documentation for DeriveInput for more information.

Next let's define our impl_hello_proc_macro function.

```rust
fn impl_hello_proc_macro(ast: &syn::DeriveInput) {
    let name = &ast.ident;
    let gen = quote!{
        impl HelloProcMacro for name {
            fn hello_proc_macro() {
                println!("Hello, the name of your type is {}", stringify!(#name))
            }
        }
    };
    gen.into()
}
```

We get an Ident struct instance containing the name (identifier) of the annotated type using `ast.ident` and assign it our new variable `name`.

The DeriveInput struct shows that when we run the impl_hello_proc_macro function on the our Mountain struct, we get the ident field with a value of "Mountain".

The quote! macro lets us define the Rust code that we want to return. The compiler expects something different to the direct result of the quote! macro’s execution, so we need to convert it to a TokenStream . We do this by calling the into method, which consumes this intermediate representation and returns a value of the required
TokenStream type.

The quote! macro also provides some very cool templating mechanics: we can enter #name , and quote! will replace it with the value in the variable name . You can even do some repetition similar to the way regular macros work. Check out the quote crate’s docs for a thorough introduction.

We want our procedural macro to generate an implementation of our HelloProcMacro trait for the type the user annotated, which we can get by using #name . The trait implementation has the one function hello_proc_macro, whose body contains the functionality we want to provide: printing Hello, the name of your type is and then the name of the annotated type. The stringify! macro used here is built into Rust. It takes a Rust expression, such as 1 + 2 , and at compile time turns the expression into a string literal, such as "1 + 2".

This is different than format! or println!, macros which evaluate the expression and then turn the result into a String. There is a possibility that the #name input might be an expression to print literally, so we use stringify!. Using stringify! also saves an allocation by converting #name to a string literal at compile time.

## Wrapping up

At this point, cargo build should complete successfully in both hello_proc_macro and hello_proc_macro_derive.

Let's see our procedural macro in action!

## Creating a binary project

We'll create a new binary project where we'll make use of our procedural macro.

```shell
cargo new mountain
```

We need to add hello_proc_macro and hello_proc_macro_derive as dependencies in the mountain crate’s `Cargo.toml`. If you’re publishing your versions of hello_proc_macro and hello_proc_macro_derive to crates.io, they would be regular dependencies; if not, you can specify them as path dependencies as follows:

```rust
hello_proc_macro = { path = "../hello_proc_macro" }
hello_proc_macro_derive = { path = "../hello_proc_macro/hello_proc_macro_derive" }
```

Next, in your main.rs file, paste this code we wrote in the beginning

```rust
use hello_proc_macro::HelloProcMacro;
use hello_proc_macro_derive::HelloProcMacro;

#[derive(HelloProcMacro)]
struct Mountain;

fn main() {
Mountain::hello_proc_macro();
}
```

Let's run our project using

```shell
cargo run
```

The implementation of the HelloProcMacro trait from the procedural macro was included without the mountain crate needing to implement it like in the code below;

```rust
use hello_proc_macro::HelloProcMacro;

struct Mountain;

impl HelloProcMacro for Mountain {
fn hello_proc_macro() {
println!("Hello, the name of your type is Mountain");
}
}

fn main() {
Mountain::hello_proc_macro();
}
```

The #[derive(HelloProcMacro)] added the trait implementation.

## The End

That's a wrap. Hats off to you for following along till the end.
