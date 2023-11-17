diesel::table! {
  posts (id) {
      id -> BigInt,
      title -> Text,
      text -> Text,
      published -> Bool,
  }
}
