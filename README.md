# README

## Work Description

| Dir          | des                                      |
| ------------ | ---------------------------------------- |
|examples|GenUI Example(Working)|
|gen|the finally framework dir(Comming Soon)|
|gen-ui|work dir(Working)|
|wiki|GenUI wiki(Comming Soon)|


- [x] Compiler
- [x] Parser
- [x] Converter
- [ ] Traits
- [ ] Macros
- [ ] Generator
  - [x] Makepad Plugin
  - [ ] Other Plugin
- [ ] GenUI VSCode Plugin
- [ ] GenUI Makepad Unified Widget Lib
- [x] [Makepad Book](https://palpus-rs.github.io/Gen-UI.github.io/)

### Makepad (see branch main)

- [x] Makepad Compiler
- [x] static page
- [ ] dyn widget (half support, now working...)
- [x] wasm
- [ ] GenUI Builtin-Widget (working...)
- [x] rust lang support (use in .gen file, hold in `<script lang="rust">` or `<script>`)

#### Widgets
- [x] Window
- [x] View
- [x] Button (todo!(button other event, click event finish))
- [x] Icon
- [x] Label 
- [x] Image
- [x] RotatedImage
- [x] Radio
- [x] Checkbox
- [x] ScrollXYView
- [x] ScrollXView
- [x] ScrollYView
- [x] SolidView
- [x] RectView
- [x] RectShadowView
- [x] RoundedView
- [x] RoundedShadowView
- [x] TextInput
- [x] DropDown
- [x] LinkLabel
- [x] FoldButton
- [x] Slider
- [x] SliderBig
- [x] Slide
  - [x] SlidesView
  - [x] SlideBody
  - [x] SlideChapter
- [x] FoldHeader
- [x] Html
- [x] Markdown
- [x] ScrollBar
- [x] ScrollBars
- [x] DesktopButton
- [x] Splitter
- [ ] Dock
- [ ] Nav

#### Props

- [x] animation
- [x] as_prop (WidgetRef)
- [x] Draw
- [x] Color
  - [x] hex
  - [x] linear
  - [x] radial
  - [x] rgb
  - [x] rgba
  - [x] shader 

#### Control

- [ ] for
- [ ] if_else

---

### HarmonyOs Ark (Empty Ability)

- [ ] Ark Compiler (working...)
- [ ] static page
- [ ] dyn widget
- [ ] GenUI Builtin-Widget
- [x] ets support (use in .gen file, hold in `<script lang="ets">`)

#### Widget
- [ ] Layout
  - [ ] Row
  - [ ] Column
  - [ ] Stack
  - [ ] RelativeContainer
  - [ ] GridRow
  - [ ] GridCol
  - [ ] List
  - [ ] Grid
  - [ ] GridItem
  - [ ] Swiper
  - [ ] Tabs
- [ ] Button
- [ ] Radio
- [ ] Toggle
- [ ] Progress
- [ ] Text
- [ ] Span
- [ ] TextInput
- [ ] TextArea
- [ ] Image
- [ ] CustomDialog
- [ ] Video
- [ ] XComponent
- [ ] Popup
- [ ] Menu

#### Others

- [ ] animation
- [ ] Navigation
- [ ] Shape
- [ ] Canvas
- [ ] Theme

## Architecture

<img src="./README/imgs/framework.png">

## DSL Design

<img src=".\README\imgs\b91eef4caddeffb49b3316304a8567f.png" alt="b91eef4caddeffb49b3316304a8567f" style="zoom:50%;" />

## Syntax Match

<img src=".\README\imgs\e3a48b59cc2fd000fa16ac14ddac999.png" alt="e3a48b59cc2fd000fa16ac14ddac999" style="zoom:50%;" />