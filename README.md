# pull_request_tutorial

Pull Request を体験してみよう！

## なにこれ

このプログラムは「いつどこで誰が何をしたゲーム」を行うものです。  
みなさんにはその元となる文章を考えていただき、それを Pull Request を使って提出してもらいます。

## いつどこで誰が何をしたゲームとは？

ゲーム参加者が「いつ」「どこで」「誰が」「何をした」かをそれぞれ分けて書き、複数集めたその文章をバラバラに組み替えて新しい文章を作るというゲームです。

参加者は、以下の template.json に従って文章を考えます。

```json
{
  "when": "",
  "where": "",
  "who": "",
  "what": ""
}
```

例えば、

A さん

```json
{
  "when": "昔々あるところに",
  "where": "川で",
  "who": "桃太郎が",
  "what": "どんぶらこしていた"
}
```

B さん

```json
{
  "when": "ある日",
  "where": "森の中で",
  "who": "クマさんが",
  "what": "姫を追いかけまわした"
}
```

C さん

```json
{
  "when": "大空洞到着後",
  "where": "最深部にて",
  "who": "クラウドたちが",
  "what": "セフィロスを始末した"
}
```

D さん

```json
{
  "when": "الآن مضى وقت طويل",
  "where": "في الجبال",
  "who": "الرجل العجوز تاكيتوري",
  "what": "مصادفة الأميرة كاجويا"
}
```

といった文章ができたとします。  
これをプログラムを使ってバラバラに組み替えます。

- ある日
- 森の中で
- クラウドたちが
- مصادفة الأميرة كاجويا

こういった文章を読んでゲラゲラ笑うのがこのゲームの楽しみ方です。

## 遊び方

### A: build 済みのファイルを使う場合

このリポジトリを clone

```
git clone https://github.com/denx-official/pull_request_tutorial
cd pull_request_tutorial
```

作業ディレクトリで build ファイルをダウンロード & 実行

```
# Windows
curl -LJO https://github.com/denx-official/pull_request_tutorial/releases/download/0.1.0/pull_request_tutorial.exe
./pull_request_tutorial.exe

# macOS
curl -LJO https://github.com/denx-official/pull_request_tutorial/releases/download/0.1.0/pull_request_tutorial_mac
./pull_request_tutorial_mac

# Linux
curl -LJO https://github.com/denx-official/pull_request_tutorial/releases/download/0.1.0/pull_request_tutorial_linux
./pull_request_tutorial_linux
```

### B: 自前で build する場合

Rust がインストールされている必要があります。

```
git clone https://github.com/denx-official/pull_request_tutorial
cd pull_request_tutorial
cargo run
```
