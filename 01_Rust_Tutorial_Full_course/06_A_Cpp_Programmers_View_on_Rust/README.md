# Rust vs C++

<table border="1">
    <tr>
    <td colspan="3" align="center"></a></td>
    </tr>
    <tr align="center">
        <td>✨</td>
        <td>Rust<a href="https://www.rust-lang.org/"><img align="left" alt="rust1" width="26px" src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" /></a></td>
        <td>C++<a href="https://en.cppreference.com/w/"><img align="left" alt="cpp" width="26px" src="https://user-images.githubusercontent.com/67513038/218466731-1c232ee4-7fe7-4c73-a201-c129e16959c2.png" /></a></td>
    </tr>
    <tr align="center">
        <td>Box</td>
        <td>Box<br></td>
        <td>Boxstd::unique_ptrC++</td>
    </tr>
    <tr align="center">
        <td>Rc</td>
        <td>Rc<br>use std::rc::Rc;</td>
        <td>std::shared_ptr</td>
    </tr>
    <tr align="center">
        <td>Primitive types</td>
        <td>Primitive types<br>(+associated methods)</td>
        <td>Primitive types</td>
    </tr>
    <tr align="center">
        <td>Structs / class</td>
        <td>Structs</td>
        <td>Structs / class</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Enumerations</td>
        <td>Enumerations</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Enumerations!</td>
        <td>Unions</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Traits</td>
        <td>(Interfaces)</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Trait Inheritance</td>
        <td>Inheritance</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Polymorphism</td>
        <td>Polymorphism</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Traits!</td>
        <td>Attributes</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Generics</td>
        <td>Templates</td>
    </tr>
</table>

<br>

<hr>

<br>

## The Ownership Model

<table border="1">
    <tr>
    <td colspan="3" align="center"></a></td>
    </tr>
    <tr align="center">
        <td>✨</td>
        <td>Rust<a href="https://www.rust-lang.org/"><img align="left" alt="rust1" width="26px" src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" /></a></td>
        <td>C++<a href="https://en.cppreference.com/w/"><img align="left" alt="cpp" width="26px" src="https://user-images.githubusercontent.com/67513038/218466731-1c232ee4-7fe7-4c73-a201-c129e16959c2.png" /></a></td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Single owner</td>
        <td>Single owner <br>or<br>Shared ownership</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Explicit</td>
        <td></td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Compiler enforced<br>- a.k.a "Borrow Checker"</td>
        <td>Developer managed</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Assignment means "move"<br>- Unless type implements Copy trait<br>- Borrowing</td>
        <td>Assignment and copy<br>construction mean "copy"<br>-- Unless R-value provided or<br>std::move used explicitly<br>(and there is a move ctor/assignment)</td>
    </tr>
</table>

<hr>

<br>

## Ownership and Borrowing

<table border="1">
    <tr>
    <td colspan="3" align="center"></a></td>
    </tr>
    <tr align="center">
        <td>✨</td>
        <td>Rust<a href="https://www.rust-lang.org/"><img align="left" alt="rust1" width="26px" src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" /></a></td>
        <td>C++<a href="https://en.cppreference.com/w/"><img align="left" alt="cpp" width="26px" src="https://user-images.githubusercontent.com/67513038/218466731-1c232ee4-7fe7-4c73-a201-c129e16959c2.png" /></a></td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Box&lt;T&gt;</td>
        <td>unique_ptr&lt;T&gt;</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Rc&lt;T&gt;, Arc&lt;T&gt;</td>
        <td>shared_ptr&lt;T&gt;</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>References (borrowing)</td>
        <td>References</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Default is immutable<br>- Add mut to declaration to mutate</td>
        <td>Default is non-const<br>- Add const to declaration</td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Multiple immutable references allowed</td>
        <td></td>
    </tr>
    <tr align="center">
        <td></td>
        <td>Mutable reference means no other references can exist at that scope</td>
        <td></td>
    </tr>
</table>

- Rust vs C++ 총정리됨
  https://maulingmonkey.com/guide/cpp-vs-rust/

https://google.github.io/comprehensive-rust/std/box.html

## Rust for C++ developers - What you need to know to get rolling with crates - Pavel Yosifovich

- https://youtu.be/k7nAtrwPhR8

<br>

# How I Wrote a Modern C++ Library in Rust

https://hsivonen.fi/modern-cpp-in-rust/

<br>

<hr>

# A C++ Programmer's View on Rust

https://youtu.be/5pdRnva-DQ4

<br>

<hr>

# Rust for C++ programmers

https://github.com/nrc/r4cppp

<br>

<hr>
