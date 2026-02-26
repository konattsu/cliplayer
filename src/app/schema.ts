import { type } from "arktype";

// ユーザー情報のスキーマ定義
export const UserSchema = type({
  id: "string",
  name: "string",
  email: "string",
  age: "number >= 0",
  role: "'admin' | 'user' | 'guest'",
  createdAt: "Date",
  "tags?": "string[]",
});

export type User = typeof UserSchema.infer;

// タスク情報のスキーマ定義
export const TaskSchema = type({
  id: "string",
  title: "string",
  description: "string",
  completed: "boolean",
  priority: "'low' | 'medium' | 'high'",
  dueDate: "Date | undefined",
});

export type Task = typeof TaskSchema.infer;

// フォーム入力のスキーマ定義
export const ContactFormSchema = type({
  name: "string > 0",
  email: "string",
  message: "string >= 10",
  "phone?": "string",
});

export type ContactForm = typeof ContactFormSchema.infer;
