---
title: Leptos + tailwindcss
cover: /images/leptos-tailwind.webp
date: 2024-04-10T10:59:31+08:00
tags:
  - RUST
draft: true
summary: leptos增加tailwindcss配置代码示例
---

1. 安装

```bash
npm install -D tailwindcss
//或者用bun？？
```

2. 初始化

```bash
npx tailwindcss init
```

3. tailwind.config.js配置中加上对\*.rs文件的扫描

```jsx
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
};
```

4. 项目根目录下放置input.css

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

5. 输出css到某个目录

```bash
npx tailwindcss -i ./input.css -o ./style/output.css --watch
```

6. 修改index.html的内容

```html
<!doctype html>
<html>
  <head>
    <link data-trunk rel="rust" data-wasm-opt="z" />
    <!-- <link data-trunk rel="icon" type="image/ico" href="/public/favicon.ico" /> -->
    <link data-trunk rel="css" href="/style/output.css" />
  </head>
  <body></body>
</html>
```
