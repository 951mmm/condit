# react 随笔

## 问题

状态管理和组件通信是 react 绕不开的坎。当两个子组件需要通信，原教主义的做法：

1. 提升状态到 LCA 父节点，将状态控制函数通过 props 送到子节点。
2. props drilling，让 props 自上而下，逐级散开
3. context, 通过 provider 寄存状态和状态控制函数，通过 consumer 获取
4. composite： 将中间层打包成一个 Wrap，把子组件当 Wrap 的 props 传入，那么子组件的 props 和父组件是平级的

我认为的弊端：

1. 传递回调函数，会造成动作和 UI 分离，比如请求一个资源需要点击按钮，按钮是子组件，资源展示界面是父组件。那么请求资源的动作就应该写在按钮里，资源的状态则写在父组件里。
2. props 爆炸，无意义的传送 prop 将导致模板代码太多
3. provider 爆炸
4. 打破标记语言的树状结构

## 我的做法

虽然将动作定义在子组件会造成数据流反向，但是我想问：单向数据流作为一个大周期是否合理？我认为只要最终 state 改变收敛，那么就视作完成一次周期即可。

我的做法：

1. props 只能传状态，不能传回调函数。如果需要回调，将 state 提升到 store
2. 在满足 1 的情况下，末端组件必须传 props，如果父组件无该 state，则从 store 获取并传送
3. 在满足 1，2 的情况下，中间组件如果 props 没有用上，则将该 state 提升到 store
4. 在满足 1， 2， 3 的情况下，如果某个状态和状态管理可以抽象(比如错误显示逻辑，pops 或者 tips)，则提升到 store

总结就是：连续向下的数据流使用 react 的 props，其他的都用 store(有种重树剖分的美)

## 原因

最近对函数式编程和有一些了解和看法，来具体聊聊 react 的设计思路和范式，来谈谈我改变设计理念的原因

### 函数式编程天生和缓冲区是互斥的

全局的可变缓冲区相当于全局变量。如果在一个函数里访问缓冲区并返回缓冲区的值，很显然这个函数的返回值是不确定的，那么就不是纯函数，因此和函数式思想相互违背

### 函数式组件的缓冲区是什么

如果定义了一个函数式组件，并且内部使用了 useState 和 UI 双向绑定之后，那么这个函数式组件就使用了缓冲区。缓冲区就是`state`。我们操作 UI`写缓冲区`，而函数式组件`读缓冲区`来响应我们的动作并展示到 UI 中。举个例子

```tsx
function Content() {
    const [show, setShow] = useState(false);

    return (
        <button onClick={() => setShow(show => !show)} />
        <p style={{ visibility: show }}>...I am content...</p>
    )
}
```

我们点击按钮设置了缓冲区`show`的值,接着 react 帮我们执行一遍这个函数，并在执行中读了缓冲区`show`的值，得到返回的`UI`也发生改变。也就是说，每次调用`Content`函数可能返回不一样的值。比如上一次调用得到的`UI`是`p>visibility=true`的，下次调用就成了`false`

### 如何巧妙避开(逃避)问题

react 的设计者脑袋一拍，如果我让缓冲区成为函数的参数，并且让缓冲区的声明尽量在高层组件中实现，底层组件只需要接收高层组件缓冲区的参数，那么是不是就算`基本上`实现函数式了呢?

接着，他们为自己在前端开辟出一个新的设计范式激动不已，并未该范式取了一个优雅的名称:`单向数据流`。举个例子

```tsx
function Content({show, setShow}) {
        return (
        <button onClick={() => setShow(show => !show)} />
        <p style={{ visibility: show }}>...I am content...</p>
    )
}

function HighLevelComponent() {
    const [HighLevelBuffer, setHighLevelBuffer] = useState({
        ...,
        show: false,
        ...,
    })

    return (
        <>
        ...
        <Content show={show} setShow={() => {
            setHighlevelBuffer(highLevelBuffer => {
                ...highLevelBuffer,
                show: !highLevelBuffer.show
            }) // 相当于函数调用 return Content({show, setShow})
        }} />
        ...
        </>
    )
}
```
从上边的代码可以看出，react的做法其实很简单：在顶层(可以是`Index.tsx`，可以是`App.tsx`)维护一个巨型缓冲区，并且让缓冲区的`getter`和`setter`通过`props`向下传送。因此整个react项目中有副作用的函数可能就只有几个，其他的只需要用`props`即可。是不是看起来很伟大?
真的，伟，伟大吗？
### 优雅or实用
看似解决了问题，其实带来了新的问题
#### 开销问题
由于react触发渲染和`setState`是绑定的，而底层组件修改的是高层组件的`state`,那么将会触发高层组件的渲染，接着整个组件树从上到下重新渲染，新的`state`通过`props`重新传给底层组件来达到更新
#### 真的消除了吗
有一些第三方全局缓冲区改变并不会触发重新渲染，并且一些人喜欢用自己定义的全局变量来当`props`或者`state`，因此react团队无奈给出了`useEffect`来让我们可以通过依赖自定`props`，从而同步缓冲区`state`,来`setState`从而触发更新。useEffect相当于一个缓冲区轮询函数，一旦依赖的缓冲区变量改变，就触发渲染

那么问题来了，如果我用`useEffect`轮询`props`，来改变本地的`state`，从而触发渲染。那么我用`state`的意义是什么?为何不直接提供`render`函数？举个例子
```tsx
// 第三方缓冲区
let show =false;
function useUnBindedState() {
    window.eventListener.on("some button triggered", () => {
        show = !show;
    })
}
function Content() {

        const show = useUnBindedSate(); // 这个hook并不会触发重新渲染

        const [render, setRender] = useState(false);

        function onRender() {
            setRender(render => !render);
        }

        useEffect(() => {
            onRender()
        }, [show])
        return (
        <button onClick={} />
        <p style={{ visibility: show }}>...I am content...</p>
    )
}
```
或者这样
```tsx
// 一些人美名其曰:优化
function Content({show}) {
    const [showInner, setShowInner] = useState(show);

    useEffect(() => {
        setShowInner(showInner => !showInner);
    }, [show])
    
    return (
        <button/>
        <p style={{ visibility: show }}>...I am content...</p>
    )
}

function HighLevelComponent() {
    const show = useRef(false);

    return (
        <>
        ...
        <Content show={show.current} />
        ...
        </>
    )
}
```
第一种代码我不多bb，第二种就纯属于脱裤子放屁。那么第二种就没必要了吗？其实并不是
#### 状态提升什么时候用
1. 当我从服务器获取了一堆数据，并且这些数据的子集是可以修改时，我们很自然的将这堆数据定义到高层组件
2. 数据只负责展示，而没有修改动作，那么需要状态提升。
3. 没有3， 其余情况考虑第三方store
4. 尽量少用useEffect来尝试同步缓冲区，既然需要同步为何不考虑合并？

以上是我的看法

以下是一些废话和原文章

### 反观历史

由于 state 本身就是副作用，即全局变量，那么引入一个全局状态管理库和 react 的哲学并不冲突。反而 react 应该摒弃限定 state 的作用域，只能通过 props 传递的做法(本身就是全局变量，规矩还这么多)。这种 props 的写法反而更多是历史问题:

### react 1.0

在 jq 时代，react 创建了虚拟 dom，本质就是通过函数的层层嵌套来构建树形结构

```html
<html>
  ...
  <body>
    <div id="root"></div>

    <script type="text/javascript">
      import { createElement } from 'react';

      function Greeting({ name }) {
        return createElement(
          'h1',
          { className: 'greeting' },
          'Hello ',
          createElement('i', null, name),
          '. Welcome!'
        );
      }

      function App() {
        return createElement(
          Greeting,
          { name: 'Taylor' }
        );
      }

      ReactDOM.createRoot(document.getElementById("root")， App)
    </script>
  </body>
</html>
```

我们可以发现能每一个函数只有 props，tag，children。状态管理怎么办呢？自己在函数作用域或者全局作用域定义全局变量不就好了。

### react 2.0

这时候出现了类的写法，react 将状态限定到类的闭包里边，那么每个组件的状态都是局部的。而正是由于局部的原因，只能单向数据流一步步往下传。但是受限于组件的类组件的生命周期是固定的，且提供了生命周期钩子函数，那么状态的改变可以定义在渲染前，也可以定义在在渲染后。举一个例子，比如我从服务器获取了文章，那么需要设置文章的内容；设置了文章的内容则需要设置文章的字数;设置了文章的字数则需要设置文章字数的颜色(假设文章是小学作文，没超过 100 字为红色，超过 100 字为绿色), 一个开关可以设置字的颜色，那么这些副作用应该写在渲染前还是渲染后?由于暴露了太多的钩子函数让管理变得复杂。

### react 3.0

函数组件，隐藏生命周期钩子函数，每次 setState 都会重渲染，因此如何合并 setState 成了 react 使用者的新的心智负担。由于 useState 其实也是全局管理，那么 state 限定在函数的作用域里显然是多此一举。

## 开销

事实证明，不用 props, state，全部存 store 开销最小，但是 M-VM-V 将坍缩为 M-M-VM-V,如果 chromium 有一天宣布内嵌 react 的运行时，交互模型将最终进化为 M-M-VM(oop 的胜利)

## 其实很简单

各种奇葩的框架本质上都没什么区别。react 的 setState 触发渲染，useEffect 就是 render 的 callback。

```ts
function useEffect(func, deps) {
    render.callbacks.push(() => {
        if (isDiff(getDepsFromState(globalState, deps), deps)) {
            const del = func();
            del();
        }
    });
}
...
useEffect(() => setState(some), [...somes])

setState -> render -> render.callbacks.map(callback => callback())
```

这便是渲染的时间循环，至于 fiber 就是将这个循环拆分，做到和浏览器的渲染并发。为何我不能在 render 之前把所有的状态都安排好呢?于是各种 store 就出现了。
