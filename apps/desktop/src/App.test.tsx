// Browser tests verify navigation and honest native-only states without mock asset data.
import { fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, test, vi } from "vitest";
import { cleanup } from "@testing-library/react";
import App from "./App";

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
});

test("S2 library exposes search and import controls without browser-only fake assets", () => {
  render(<App />);
  fireEvent.click(screen.getByRole("link", { name: /图标库/ }));
  expect(screen.getByRole("searchbox", { name: "搜索素材" })).toBeTruthy();
  expect(screen.getByRole("button", { name: "＋ 导入文件" })).toHaveProperty(
    "disabled",
    true,
  );
  expect(screen.getByRole("heading", { name: "图标库还是空的" })).toBeTruthy();
});

test("browser preview exposes all six routes and identifies native-only project data", async () => {
  render(<App />);
  expect(screen.getByRole("navigation").querySelectorAll("a")).toHaveLength(6);
  fireEvent.click(screen.getByRole("link", { name: /工作台/ }));
  expect(await screen.findByRole("status")).toHaveProperty(
    "textContent",
    "浏览器预览：项目管理需要桌面安装版，浏览器中不会创建假数据。",
  );
  for (const label of [
    "图标库",
    "组件库",
    "Design Tokens",
    "设计图库",
    "收集箱",
  ]) {
    fireEvent.click(screen.getByRole("link", { name: new RegExp(label) }));
    expect(screen.getByRole("heading", { name: label })).toBeTruthy();
  }
});
