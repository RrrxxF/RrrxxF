// S1 browser test verifies route navigation without pretending SQLite is available.
import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, test, vi } from "vitest";
import { cleanup } from "@testing-library/react";
import App from "./App";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

test("browser preview exposes all six routes and identifies native-only project data", async () => {
  render(<App />);
  expect(screen.getByRole("navigation").querySelectorAll("a")).toHaveLength(6);
  expect(await screen.findByRole("status")).toHaveProperty(
    "textContent",
    "浏览器预览：项目管理需要桌面安装版，浏览器中不会创建假数据。",
  );
  for (const label of ["图标库", "组件库", "Design Tokens", "设计图库", "收集箱"]) {
    fireEvent.click(screen.getByRole("link", { name: new RegExp(label) }));
    expect(screen.getByRole("heading", { name: label })).toBeTruthy();
  }
});
