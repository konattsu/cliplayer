import { Component, signal, computed, inject } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import {
  CommonModule,
  DatePipe,
  UpperCasePipe,
  CurrencyPipe,
} from "@angular/common";
import {
  FormBuilder,
  FormGroup,
  Validators,
  ReactiveFormsModule,
} from "@angular/forms";
import {
  CdkDragDrop,
  DragDropModule,
  moveItemInArray,
} from "@angular/cdk/drag-drop";
import { ScrollingModule } from "@angular/cdk/scrolling";
import { Dialog } from "@angular/cdk/dialog";

// Angular Material imports
import { MatButtonModule } from "@angular/material/button";
import { MatCardModule } from "@angular/material/card";
import { MatInputModule } from "@angular/material/input";
import { MatFormFieldModule } from "@angular/material/form-field";
import { MatSelectModule } from "@angular/material/select";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatTableModule } from "@angular/material/table";
import { MatChipsModule } from "@angular/material/chips";
import { MatIconModule } from "@angular/material/icon";
import { MatTabsModule } from "@angular/material/tabs";
import { MatExpansionModule } from "@angular/material/expansion";
import { MatBadgeModule } from "@angular/material/badge";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MatSlideToggleModule } from "@angular/material/slide-toggle";

// Arktype schemas
import {
  UserSchema,
  TaskSchema,
  ContactFormSchema,
  type User,
  type Task,
} from "./schema";

@Component({
  selector: "app-root",
  imports: [
    RouterOutlet,
    CommonModule,
    ReactiveFormsModule,
    DragDropModule,
    ScrollingModule,
    MatButtonModule,
    MatCardModule,
    MatInputModule,
    MatFormFieldModule,
    MatSelectModule,
    MatCheckboxModule,
    MatTableModule,
    MatChipsModule,
    MatIconModule,
    MatTabsModule,
    MatExpansionModule,
    MatBadgeModule,
    MatProgressSpinnerModule,
    MatSlideToggleModule,
    DatePipe,
    UpperCasePipe,
    CurrencyPipe,
  ],
  templateUrl: "./app.html",
  styleUrl: "./app.scss",
})
export class App {
  private fb = inject(FormBuilder);
  private dialog = inject(Dialog);

  // Signals を使った状態管理
  protected readonly title = signal("Angular Demo App");
  protected readonly loading = signal(false);
  protected readonly tasks = signal<Task[]>([
    {
      id: "1",
      title: "Angular Material の導入",
      description: "Material コンポーネントを使用してUIを構築",
      completed: true,
      priority: "high",
      dueDate: new Date("2026-02-20"),
    },
    {
      id: "2",
      title: "Arktype の統合",
      description: "型安全なバリデーションを実装",
      completed: true,
      priority: "high",
      dueDate: new Date("2026-02-21"),
    },
    {
      id: "3",
      title: "CDK の活用",
      description: "Drag & Drop やVirtual Scrollを実装",
      completed: false,
      priority: "medium",
      dueDate: new Date("2026-02-28"),
    },
  ]);

  protected readonly users = signal<User[]>([
    {
      id: "u1",
      name: "太郎",
      email: "taro@example.com",
      age: 25,
      role: "admin",
      createdAt: new Date("2025-01-15"),
      tags: ["developer", "frontend"],
    },
    {
      id: "u2",
      name: "花子",
      email: "hanako@example.com",
      age: 30,
      role: "user",
      createdAt: new Date("2025-02-10"),
      tags: ["designer", "ui/ux"],
    },
  ]);

  // Computed signals
  protected readonly completedTasks = computed(
    () => this.tasks().filter((t) => t.completed).length,
  );

  protected readonly totalTasks = computed(() => this.tasks().length);

  protected readonly completionRate = computed(() => {
    const total = this.totalTasks();
    if (total === 0) return 0;
    return Math.round((this.completedTasks() / total) * 100);
  });

  // Reactive form with validators
  protected readonly contactForm: FormGroup;
  protected readonly validationErrors = signal<string[]>([]);

  // Table columns
  protected readonly displayedColumns = [
    "name",
    "email",
    "age",
    "role",
    "createdAt",
  ];
  protected readonly price = signal(12345.67);

  constructor() {
    // フォームの初期化
    this.contactForm = this.fb.group({
      name: ["", [Validators.required, Validators.minLength(1)]],
      email: ["", [Validators.required, Validators.email]],
      message: ["", [Validators.required, Validators.minLength(10)]],
      phone: [""],
    });
  }

  // タスクの完了状態をトグル
  protected toggleTask(taskId: string): void {
    this.tasks.update((tasks) =>
      tasks.map((task) =>
        task.id === taskId ? { ...task, completed: !task.completed } : task,
      ),
    );
  }

  // Drag & Drop ハンドラー
  protected onDrop(event: CdkDragDrop<Task[]>): void {
    const items = [...this.tasks()];
    moveItemInArray(items, event.previousIndex, event.currentIndex);
    this.tasks.set(items);
  }

  // フォーム送信
  protected submitForm(): void {
    if (this.contactForm.invalid) {
      this.validationErrors.set(["フォームに入力エラーがあります"]);
      return;
    }

    const formValue = this.contactForm.value;

    // Arktype でバリデーション
    const result = ContactFormSchema(formValue);

    // Arktypeの結果は成功時にはdataプロパティを持ち、失敗時には持たない
    if ("data" in result) {
      // バリデーション成功
      const validData = result.data as {
        name: string;
        email: string;
        message: string;
        phone?: string;
      };
      this.validationErrors.set([]);
      console.log("Form submitted successfully:", validData);
      alert(`送信成功！\n名前: ${validData.name}\nメール: ${validData.email}`);
      this.contactForm.reset();
    } else {
      // バリデーションエラー
      this.validationErrors.set([result.toString()]);
      console.error("Arktype validation failed:", result);
    }
  }

  // 新しいユーザーを追加（Arktype検証付き）
  protected addUser(): void {
    const newUser = {
      id: `u${this.users().length + 1}`,
      name: "次郎",
      email: "jiro@example.com",
      age: 28,
      role: "user" as const,
      createdAt: new Date(),
      tags: ["tester"],
    };

    const result = UserSchema(newUser);

    if ("data" in result) {
      const validUser = result.data as User;
      this.users.update((users) => [...users, validUser]);
      console.log("User added successfully:", validUser);
    } else {
      console.error("User validation failed:", result);
      alert("ユーザー追加に失敗しました");
    }
  }

  // ローディング状態をトグル
  protected toggleLoading(): void {
    this.loading.update((v) => !v);
  }
}
