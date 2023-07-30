## Visitor and Double Dispatch

Let's take a look at following class hierarchy of geometric shapes (beware the pseudocode):
```c++
 1 interface Graphic is
 2     method draw()
 3 
 4 class Shape implements Graphic is
 5     field id
 6     method draw()
 7     // ...
 8 
 9 class Dot extends Shape is
10     field x, y
11     method draw()
12     // ...
13 
14 class Circle extends Dot is
15     field radius
16     method draw()
17     // ...
18 
19 class Rectangle extends Shape is
20     field width, height
21     method draw()
22     // ...
23 
24 class CompoundGraphic implements Graphic is
25     field children: array of Graphic
26     method draw()
27     // ...
```

The code works fine and the app is in production. But one day you decided to make an export feature. The export code 
would look alien if placed in these classes. So instead of adding export to all classes of this hierarchy you decided 
to create a new class, external to the hierarchy and put all the export logic inside. The class would get methods for 
exporting public state of each object into XML strings:
```c++
 1 class Exporter is
 2     method export(s: Shape) is
 3         print("Exporting shape")
 4     method export(d: Dot)
 5         print("Exporting dot")
 6     method export(c: Circle)
 7         print("Exporting circle")
 8     method export(r: Rectangle)
 9         print("Exporting rectangle")
10     method export(cs: CompoundGraphic)
11         print("Exporting compound")
```

The code looks good, but let's try it out:
```c++
 1 class App() is
 2     method export(shape: Shape) is
 3         Exporter exporter = new Exporter()
 4         exporter.export(shape);
 5
 6 app.export(new Circle());
 7 // Unfortunatelly, this will output "Exporting shape".
```

Wait! Why?!


### Thinking as a compiler
Note: the following information is true for the most modern object-oriented programming languages 
(Java, C#, PHP, and others).

#### Late/dynamic binding
Pretend that you're a compiler. You have to decide how to compile the following code:
```c++
 1 method drawShape(shape: Shape) is
 2     shape.draw();
```
Let's see... the `draw` method defined in `Shape` class. Wait a sec, but there are also four subclasses that override 
this method. Can we safely decide which of the implementations to call here? It doesn't look so. The only way to know 
for sure is to launch the program and check the class of an object passed to the method. The only thing we know for 
sure is that object **will have** implementation of the `draw` method.

So the resulting machine code will be checking class of the object passed to the `shape` parameter and picking the 
`draw` implementation from the appropriate class.

Such a dynamic type check is called late (or dynamic) binding:
- **Late**, because we link object and its implementation after compilation, at runtime.
- **Dynamic**, because every new object might need to be linked to a different implementation.


#### Early/static binding
Now, let's "compile" following code:
```c++
 1 method exportShape(shape: Shape) is
 2     Exporter exporter = new Exporter()
 3     exporter.export(shape);
```
Everything is clear with the second line: the `Exporter` class doesn't have a custom constructor, so we just 
instantiate an object. What's about the `export` call? The `Exporter` has five methods with the same name that differ 
with parameter types. Which one to call? Looks like we're going to need a dynamic binding here as well.

But there's another problem. What if there's a shape class that doesn't have appropriate `export` method in `Exporter` 
class? For instance, an `Ellipse` object. The compiler can't guarantee that the appropriate overloaded method exists 
in contrast with overridden methods. The ambiguous situation arises which a compiler can't allow.

Therefore, compiler developers use a safe path and use the early (or static) binding for overloaded methods:
- **Early** because it happens at compile time, before the program is launched.
- **Static** because it can't be altered at runtime.

Let's return to our example. We're sure that the incoming argument will be of `Shape` hierarchy: either the `Shape` 
class or one of its subclasses. We also know that `Exporter` class has a basic implementation of the export that 
supports `Shape` class: `export(s: Shape)`.

That's the only implementation that can be safely linked to a given code without making things ambiguous. That's why 
even if we pass a `Rectangle` object into `exportShape`, the exporter will still call an `export(s: Shape)` method.



### Double dispatch
**Double dispatch** is a trick that allows using dynamic binding alongside with overloaded methods. Here how it's done:
```c++
 1 class Visitor is
 2     method visit(s: Shape) is
 3         print("Visited shape")
 4     method visit(d: Dot)
 5         print("Visited dot")
 6 
 7 interface Graphic is
 8     method accept(v: Visitor)
 9 
10 class Shape implements Graphic is
11     method accept(v: Visitor)
12         // Compiler knows for sure that `this` is a `Shape`.
13         // Which means that the `visit(s: Shape)` can be safely called.
14         v.visit(this)
15 
16 class Dot extends Shape is
17     method accept(v: Visitor)
18         // Compiler knows that `this` is a `Dot`.
19         // Which means that the `visit(s: Dot)` can be safely called.
20         v.visit(this)
21 
22 
23 Visitor v = new Visitor();
24 Graphic g = new Dot();
25 
26 // The `accept` method is overriden, not overloaded. Compiler binds it
27 // dynamically. Therefore the `accept` will be executed on a class that
28 // corresponds to an object calling a method (in our case, the `Dot` class).
29 g.accept(v);
30
31 // Output: "Visited dot"
```

#### Afterword
Even though the [**Visitor**](../10_visitor) pattern is built on the double dispatch principle, that's not its primary 
purpose. Visitor lets you add "external" operations to a whole class hierarchy without changing the existing code of 
these classes.
