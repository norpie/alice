let chats = [
  {
    date_group: "Today",
    conversations: [
      {
        title: "Lorem ipsum dolor sit amet",
        id: "1",
      },
      {
        title: "Consectetur adipiscing",
        id: "2",
      },
      {
        title: "Sed do eiusmod tempor incididunt",
        id: "3",
      },
      {
        title: "Ut labore et dolore magna aliqua",
        id: "4",
      },
    ],
  },
  {
    date_group: "Yesterday",
    conversations: [
      {
        title: "Ut enim ad minim",
        id: "5",
      },
      {
        title: "Quis nostrud exercitation ullamco",
        id: "6",
      },
      {
        title: "Laboris nisi ut aliquip ex ea commodo",
        id: "7",
      },
      {
        title: "Consequat duis",
        id: "8",
      },
    ],
  },
  {
    date_group: "Last Week",
    conversations: [
      {
        title: "In reprehenderit in voluptate",
        id: "9",
      },
      {
        title: "Velit esse cillum dolore eu fugiat",
        id: "10",
      },
      {
        title: "Nulla pariatur",
        id: "11",
      },
      {
        title: "Excepteur sint occaecat cupidatat",
        id: "12",
      },
    ],
  },
  {
    date_group: "Last Month",
    conversations: [
      {
        title: "Non proident sunt in culpa",
        id: "13",
      },
      {
        title: "Lorem ipsum dolor sit amet",
        id: "14",
      },
      {
        title: "Consectetur adipiscing",
        id: "15",
      },
      {
        title: "Sed do eiusmod tempor incididunt",
        id: "16",
      },
    ],
  },
  {
    date_group: "3+ Months Ago",
    conversations: [
      {
        title: "Ut labore et dolore magna aliqua",
        id: "17",
      },
      {
        title: "Ut enim ad minim",
        id: "18",
      },
      {
        title: "Quis nostrud exercitation ullamco",
        id: "19",
      },
      {
        title: "Laboris nisi ut aliquip ex ea commodo",
        id: "20",
      },
    ],
  },
  {
    date_group: "Older",
    conversations: [
      {
        title: "Consequat duis",
        id: "21",
      },
      {
        title: "In reprehenderit in voluptate",
        id: "22",
      },
      {
        title: "Velit esse cillum dolore eu fugiat",
        id: "23",
      },
      {
        title: "Nulla pariatur",
        id: "24",
      },
    ],
  },
];

const conversation = [
  {
    role: "user",
    message:
      "How do you make an async trait in Rust and implement it for a struct?",
  },
  {
    role: "assistant",
    message:
      "You can make an async trait in Rust by using the async_trait crate. Here's an example:\n\n```rust\nuse async_trait::async_trait;\n\n#[async_trait]\ntrait MyTrait {\n    async fn my_method(&self);\n}\n```\n\nYou can then implement this trait for a struct like this:\n\n```rust\nstruct MyStruct;\n\n#[async_trait]\nimpl MyTrait for MyStruct {\n    async fn my_method(&self) {\n        // Your implementation here\n    }\n}\n```",
  },
];

export default { chats, conversation };
