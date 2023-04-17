# react 随笔

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

## 反观历史
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

函数组件，隐藏生命周期钩子函数，每次setState都会重渲染，因此如何合并setState成了react使用者的新的心智负担。由于useState其实也是全局管理，那么state限定在函数的作用域里显然是多此一举。


## 我的做法
虽然将动作定义在子组件会造成数据流反向，但是我想问：单向数据流作为一个大周期是否合理？我认为只要最终 state 改变收敛，那么就视作完成一次周期即可。

我的做法：

1. props 只能传状态，不能传回调函数。如果需要回调，将 state 提升到 store
2. 在满足 1 的情况下，末端组件必须传 props，如果父组件无该 state，则从 store 获取并传送
3. 在满足 1，2 的情况下，中间组件如果 props 没有用上，则将该 state 提升到 store
4. 在满足 1， 2， 3 的情况下，如果某个状态和状态管理可以抽象(比如错误显示逻辑，pops 或者 tips)，则提升到 state

总结就是：连续向下的数据流使用 react 的 props，其他的都用 store(有种重树剖分的美)

## 开销
事实证明，不用props, state，全部存store开销最小，但是M-VM-V将坍缩为M-M-VM-V,如果chromium有一天宣布内嵌react的运行时，交互模型将最终进化为M-M-VM(oop的胜利)


## 其实很简单
各种奇葩的框架本质上都没什么区别。react的setState触发渲染，useEffect就是render的callback。
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
这便是渲染的时间循环，至于fiber就是将这个循环拆分，做到和浏览器的渲染并发。为何我不能在render之前把所有的状态都安排好呢?于是各种store就出现了。