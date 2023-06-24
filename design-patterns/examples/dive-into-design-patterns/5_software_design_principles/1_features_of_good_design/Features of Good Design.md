## Features of Good Design 优秀设计的特征
Before we proceed to the actual patterns, let's discuss the process of designing software architecture: things to aim 
for and things you'd better avoid.   
译: 在开始学习设计模式之前，我们先来看看软件架构的设计过程，了解一下需要达成的目标以及需要尽量避免的陷阱。


### :recycle: Code reuse 代码复用
Cost and time are two of the most valuable metrics when developing any software product. Less time in development means 
entering the market earlier than competitors. Lower development costs mean more money is left for marketing and a 
broader reach to potential customers.   
译: 无论开发何种软件产品，成本和时间都是最重要的两个衡量指标。较短的开发时间意味着可以比竞争对手更早地进入市场。较低的开发成本意味着可以留出更多的
营销资金，以便能覆盖更广泛地潜在客户。

**Code reuse** is one of the most common ways to reduce development costs. The intent is pretty obvious: instead of 
developing something over and over from scratch, why don't we reuse existing code in new projects?   
译: **代码复用**是最常用的减少开发成本的方式之一。其意图非常明显: 与其反复从头开始开发，为什么不在新项目中重用已有的代码呢?

The idea looks great on paper, but it turns out that making existing code work in a new context usually takes extra 
effort. Tight coupling between components, dependencies on concrete classes instead of interfaces, hardcoded 
operations—all of these reduces the flexibility of the code and makes it harder to reuse it.   
译: 表面上看这个想法很棒，但实际上要让已有的代码在全新的上下文中正常工作，通常需要付出额外的努力。紧密耦合的组件、依赖于具体的类而非接口以及硬编码，
这些都会降低代码的灵活性，且使得复用这些代码变得更加困难。

Using design patterns is one way to increase the flexibility of software components and make them easier to reuse. 
However, this sometimes comes at the price of making the components more complicated.   
译: 使用设计模式是增加软件组件灵活性并使其易于复用的方式之一。但是有时，这也会让组件变得更加复杂。

Here's a price of wisdom from [Erich Gamma](https://refactoring.guru/gamma-interview), one of the founding fathers of 
design patterns, about the role of design patterns in code reuse: 
> I see three levels of reuse.
> 
> At the lowest level, you reuse classes: class library, containers, maybe some class "teams" like container/iterator.
> 
> Frameworks are at the highest level. They really try to distill your design decisions.
> They identify the key abstractions for solving a problem, represent them by classes and define relationships between them.
> JUnit is a small framework, for example. It is the "Hello, world" of frameworks.
> It has Test, TestCase, TestSuite and relationships defined.
> 
> A framework is typically larger-grained than just a single class.
> Also, you hook into frameworks by subclassing somewhere.
> They use the so-called Hollywood principle of "don't call us, we'll call you."
> The framework lets you define your custom behavior, and it will call you when it's your turn to do something.
> Same with JUnit, right?
> It calls you when it wants to execute a test for you, but the rest happens in the framework. 
> 
> There also is a middle level. This is where I see patterns.
> Design patterns are both smaller and more abstract than frameworks.
> They're really a description about how a couple of classes can relate to and interact with each other.
> The level of reuse increases when you move from classes to patterns and finally frameworks.
> 
> What is nice about this middle layer is that patterns offer reuse in a way that is less risky than frameworks.
> Building a framework is high-risk and a significant investment.
> Patterns let you reuse design ideas and concepts independently of concrete code.

译: 设计模式创始人之一的[埃里希·伽玛](https://refactoringguru.cn/gamma-interview)，在谈到设计模式在代码复用中的角色时说:
> 我认为复用有3个层次。
> 
> 在最底层，你可以复用类: 类库、容器，也许还有一些类的"团队"(例如容器/迭代器)。
> 
> 在最顶层的是框架。它们确实能帮你简化你的设计。对于待解决的问题，它们能识别出关键的抽象，然后用类来表示这些概念并定义其之间的关系。例如，JUnit
> 是一个小型框架。它是框架中的"Hello world"。它定义了Test、TestCase以及TestSuite这几个类及其关系。
> 
> 通常框架比单个类的颗粒度要大。你可以通过在某处构建子类来与框架建立联系。这些子类信奉所谓的好莱坞原则，即"别给我们打电话，我们会给你打电话的"。
> 框架允许你自定义自己的行为，并会在需要之时告知你。这和JUnit一样，对吧? 当它希望执行测试时就会告诉你，但其他的一切都发生在框架内。
> 
> 还有一个中间层。这也是我认为设计模式所处的位置。设计模式比框架更小且更抽象。它们实际上是对一组类的关系及其交互方式的描述。当你从类转向设计模式并
> 最终抵达框架的过程中，复用的程度是在不断增加的。
> 
> 中间层的优点在于设计模式提供的复用方式要比框架的风险小。打造一款框架，投入大且风险高。设计模式则让你独立于具体的代码来复用设计思想和理念。


### :snowflake: Extensibility 扩展性
**Change** is the only constant thing in a programmer's life.
- You released a video game for Windows, but now people ask for a macOS version.
- You created a GUI framework with square buttons, but several months later round buttons become a trend.
- You designed a brilliant e-commerce website architecture, but just a month later, customers ask for a feature that 
would let them accept phone orders.   

译: **变化**是程序员生命中唯一不变的事情。
- 你在Windows平台上发布了一款游戏，但是现在人们想要macOS的版本。
- 你创建了一个使用方形按钮的GUI框架，但是几个月后圆形按钮开始流行起来。
- 你搭建了一款优秀的电子商务网站，但是仅仅一个月后，客户就要求新增接受电话订购的功能。

Each software developer has dozens of similar stories. There are several reasons why this happens.   
译: 每位软件开发者都经历过许多相似的故事。导致它们发生的原因有以下。

First, we understand the problem better once we start to solve it. Often by the time you finish the first version of 
an app, you're ready to rewrite it from scratch because now you understand many aspects of the problem much better. 
You have also grown professionally, and your own code now looks like crap.   
译: 首先，我们在开始着手解决问题后才能更好地理解问题。通常在完成了第一版的程序后，你就做好了从头开始重写代码的准备，因为现在你已经能在很多方面
更好地理解问题了。同时在专业水平上也有所提高，之前的代码现在看上去可能会显得很糟糕。

Something beyond your control has changed. This is why so many dev teams pivot from their original ideas into something
new. Everyone who relied on Flash in an online application has been reworking or migrating their code as browser after
browser drops support for Flash.   
译: 其次是在你掌控之外的某些事情发生了改变。这也是导致许多开发团队改变最初想法的原因。在网络应用中使用Flash的开发者都必须重新开发或移植代码，
因为不断地有浏览器停止对Flash格式的支持。

The third reason is that the goalposts move. Your client was delighted with the current version of the application, but 
now sees eleven "little" changes he'd like, so it can do other things he never mentioned in the original planning 
sessions. These aren't frivolous changes: your excellent first version has shown him that even more is possible.   
译: 第三个原因是需求的改变。之前你的客户对当前版本的应用非常满意，但是现在希望对应用进行11处 "小小"的改动，使其可以实现原始计划阶段中完全没有
提到的功能。这些都不是轻率地改变: 你的第一个完美版本向他展示了更多的可能性。

> There's a bright side: if someone asks you to change something in your app, that means someone still cares about it.
> 
> 译: 这也有好的一面: 如果有人要求你对应用进行修改，至少说明还有人关心它。

That's why
all seasoned developers try to provide for possible future changes when designing an application's architecture.   
译: 因此在设计应用的架构时，所有有经验的开发者会尽量选择支持未来任何可能变更的方式。
