## What's Design Patterns? 什么是设计模式？
**Design patterns** are typical solutions to commonly occurring problems in software design. They are like pre-made 
blueprints that you can customize to solve a recurring design problem in your code.   
译: **设计模式**是软件设计中常见问题的典型解决方案。它们就像能根据需求进行调整的预制蓝图，可用于解决代码中反复出现的设计问题。

You can't just find a pattern and copy it into your program, the way you can with off-the-shelf functions or libraries. 
The pattern is not a specific piece of code, but a general concept for solving a particular problem. You can follow the 
pattern details and implement a solution that suits the realities of your own program.   
译: 设计模式与方法或库的使用方式不同，你很难直接在自己的程序中套用某个设计模式。模式并不是一段特定的代码，而是解决特定问题的一般性概念。你可以
根据模式来实现符合自己程序实际所需的解决方案。

Patterns are often confused with algorithms, because both concepts describe typical solutions to some known problems. 
While an algorithm always defines a clear set of actions that can achieve some goal, a pattern is a more high-level 
description of a solution. The code of the same pattern applied to two different programs may be different.   
译: 人们常常会混淆模式和算法，因为二者在概念上都是已知特定问题的典型解决方案。但算法总是明确定义达成特定目标所需的一系列步骤，而模式则是对解决
方案的更高层次描述。同一模式应用于两个不同的程序，其实现代码可能会不一样。

An analogy to an algorithm is a cooking recipe: both have clear steps to achieve a goal. On the other hand, a pattern 
is more like a blueprint: you can see what the result and its features are, but the exact order of implementation is up 
to you.   
译: 算法更像是菜谱: 都提供达成目标的明确步骤。而模式更像是蓝图: 你可以看到最终的结果和模式的功能，但需要你自己确定实现的步骤。

### :books: What does the pattern consist of? 模式包含哪些内容?
Most patterns are described very formally, so people can reproduce them in many contexts. Here are the sections that are 
usually present in a pattern description:
- **Intent** of the pattern briefly describes both the problem and the solution.
- **Motivation** further explains the problem and the solution the pattern makes possible.
- **Structure** of classes shows each part of the pattern and how they are related.
- **Code example** in one of the popular programming languages makes it easier to grasp the idea behind the pattern.

译: 大部分模式的描述都会遵循特定的形式，以便在不同情况下使用。模式的描述通常会包含以下部分:   
- **意图**部分会简要地描述问题和解决方案。
- **动机**部分会进一步地解释问题并说明模式是如何提供解决方案的。
- **结构**部分会模式的各个部分以及它们之间的关系。
- **代码示例**部分会提供各种流行编程语言的实现，已使读者更好地理解模式背后的思想。

Some pattern catalogs list other useful details, such as applicability of the pattern, implementation steps, and 
relations with other patterns.   
译: 部分模式介绍中还列出了其他的一些实用细节，例如模式的适用性、实现步骤以及与其他模式的关系。

### :abcd: Classification of patterns 模式的分类
Design patterns differ by their complexity, level of detail and scale of applicability to the entire system being 
designed. I like the analogy to road construction: you can make an intersection safer by either installing some traffic 
lights or building an entire multi-level interchange with underground passages for pedestrians.   
译: 不同设计模式在复杂度、细节层次以及在整个系统中的应用范围等方面各不相同。我喜欢将其比作道路的修建: 如果你希望让十字路口更加安全，那么可以
安装一些交通信号灯，或者修建有行人地下通道的多层互通式立交桥。

The most basic and low-level patterns are often called idioms. They usually apply only to a single programming language.   
译: 最基础最底层的模式通常被称为惯用技巧。这类模式一般只能在一种编程语言中使用。

The most universal and high-level patterns are architectural patterns. Developers can implement these patterns in 
virtually any language. Unlike other patterns, they can be used to design the architecture of an entire application.   
译: 最通用最高层的模式是架构模式。开发者可以在任何编程语言中使用这类模式。与其他模式不同，它们可用于设计整个应用程序的架构。

In addition, all patterns can be categorized by their intent, or purpose. This book covers three main groups of patterns:
- **Creational patterns** provide object creation mechanisms that increase flexibility and reuse of existing code.
- **Structural patterns** explain how to assemble objects and classes into larger structures, while keeping the structures 
flexible and efficient.
- **Behavioral patterns** take care of effective communication and the assignment of responsibilities between objects.   

译: 此外，所有模式可以根据其意图或目的来分类。本书覆盖了三种主要的模式类别:
- **创建型模式**提供了创建对象的机制，其增加了已有代码的灵活性和可复用性。
- **结构型模式**介绍了如何将对象和类组装成较大的结构，同时保持结构的灵活性和高效性。
- **行为型模式**负责对象间的高效沟通和职责委派。

### :green_book: Who invented patterns? 谁发明了设计模式?
That's a good, but not a very accurate, question. Design patterns aren't obscure, sophisticated concepts—quite the 
opposite. Patterns are typical solutions to common problems in object-oriented design. When a solution gets repeated 
over and over in various projects, someone eventually puts a name to it and describes the solution in detail. That's 
basically how a pattern gets discovered.   
译: 这是一个好问题，但有点不太准确。设计模式并不是晦涩的、复杂的概念——事实恰恰相反。模式是面向对象设计中常见问题的典型解决方案。同样的解决
方案在各种项目中得到了反复使用，所以最终有人给它们起了名字，并对其进行了详细描述。这基本上就是模式被发现的历程了。

The concept of patterns was first described by Christopher Alexander in [*A Pattern Language: Towns, Buildings,
Construction*](https://refactoring.guru/pattern-language-book). The book describes a "language" for designing the urban 
environment. The units of this language are patterns. They may describe how high windows should be, how many levels a 
building should have, how large green areas in a neighborhood are supposed to be, and so on.   
译: 模式的概念是由克里斯托佛·亚历山大在其著作[《建筑模式语言》](https://refactoringguru.cn/pattern-language-book)中首次提出的。本书介绍
了城市设计的"语言"，而该语言的基本单元就是模式。 它们可以描述窗户应该在多高、一座建筑应该有多少层以及一片街区应该有多大面积的植被等等。

The idea was picked up by four authors: Erich Gamma, John Vlissides, Ralph Johnson, and Richard Helm. In 1994, they 
published [*Design Patterns: Elements of Reusable Object-Oriented Software*](https://refactoring.guru/gof-book), in which 
they applied the concept of design patterns to programming. The book featured 23 patterns solving various problems of 
object-oriented design and became a bestseller very quickly. Due to its lengthy name, people started to call it 
"the book by the gang of four" which was soon shortened to simply "the GOF book".   
译: 埃里希·伽玛、约翰·弗利赛德斯、拉尔夫·约翰逊和理查德·赫尔姆这四位作者接受了模式的概念。1994年，他们出版了
[《设计模式: 可复用面向对象软件的基础》](https://refactoringguru.cn/gof-book)一书，将设计模式的概念应用到程序开发领域中。该书提供了23个
模式来解决面向对象设计中的各种问题，很快便成为了畅销书。由于书名太长，人们将其简称为"四人帮(Gang of Four, GoF)的书"，并且很快进一步简化
为"GoF的书"。

Since then, dozens of other object-oriented patterns have been discovered. The "pattern approach" became very popular 
in other programming fields, so lots of other patterns now exist outside object-oriented design as well.   
译: 此后，人们又发现了其他几十种面向对象的模式。 "模式方法" 开始在其他编程领域中流行起来，因此也存在许多非面向对象设计的模式。

### :warning: Criticism of patterns 对设计模式的批评
It seems like only lazy people haven't criticized design patterns yet. Let's take a look at the most typical arguments 
against using patterns.   
译: 目前似乎只有懒人还没有批评设计模式。让我们看一下典型的批评使用设计模式的观点。

#### Kludges for a weak programming language 弱编程语言的不成熟方案
Usually the need for patterns arises when people choose a programming language or a technology that lacks the necessary 
level of abstraction. In this case, patterns become a kludge that gives the language much-needed super-abilities.   
译: 通常，当人们选择某种缺乏必要的抽象级别的编程语言或技术时，就会产生对设计模式的需要。在这种情况下，设计模式就变成了一种不成熟方案，可以给编程
语言提供急需的超能力。

For example, the Strategy pattern can be implemented with a simple anonymous (lambda) function in most modern 
programming languages.   
译: 比如，策略模式在现代大多数编程语言中可以被实现成简单的匿名函数(lambda)。

#### Inefficient solutions 低效的解决方案
Patterns try to systematize approaches that are already widely used. This unification is viewed by many as a dogma, 
and they implement patterns "to the letter", without adapting them to the context of their project.   
译: 设计模式试图将已经广泛使用的方法系统化。这种统一方式被许多人视为是一种教条，因为他们 "教条地" 应用模式，而不考虑他们的项目背景。

#### Unjustified use 不当使用
> If all you have is a hammer, everything looks like a nail.
> 如果你只有一把锤子，那么所有东西看起来都像是钉子

This is the problem that haunts many novices who have just familiarized themselves with patterns. Having learned about 
patterns, they try to apply them everywhere, even in situations where simpler code would do just fine.   
译: 这个问题是许多新手刚刚熟悉设计模式时会遇到的。在学习了设计模式之后，他们试图把设计模式应用到所有地方，即使在更简单的代码能执行得很好的情况下。
