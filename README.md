# Amazing Compression Tech (Joke)

このリポジトリは、画像をアップロードするだけで0バイトのファイルが返ってくる「圧縮ページ」をAstroで構築したものです。名前と拡張子はそのまま、サイズは100％圧縮されました。

## 🛠️ 特徴

- Astroで静的サイトとして構築
- 出力先を `./docs` に設定し GitHub Pages に対応
- 日本語/英語のi18n対応（URLパラメータまたはブラウザ言語で切り替え）
- カスタム404ページを用意
- クライアントサイドで0バイトダウンロードを生成* 圧縮ロジックは TypeScript で `src/lib/compression.ts` に分離、公開用に `public/compression.js` を配布- グラデーション背景など少しこだわったデザイン

## 📦 使用方法

1. 依存関係をインストール: `npm install`
2. 開発サーバー起動: `npm run dev`
3. 本番ビルド: `npm run build`（成果物は `./docs/` に生成されます）

## 🚀 GitHub Pagesデプロイ

`astro.config.mjs` にはすでに以下の設定が含まれています。

```js
export default defineConfig({
  output: 'static',
  dist: 'docs',
  // base: '/amazing-compression-tech/',
  // site: 'https://<your-user>.github.io/amazing-compression-tech',
});
```

プロジェクトページとして使う場合は `base` と `site` を適宜設定し、`docs/` を `gh-pages` ブランチにプッシュしてください。

## 🌐 言語切り替え

ページ上部のセレクタまたは `?lang=ja` / `?lang=en` のクエリで言語を切り替えられます。

---

Enjoy the compression! (100％ completed 😉)
