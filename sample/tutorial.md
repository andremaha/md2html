Introduction
------------

Markdown is a markup language designed to be simple enough to let anyone write structured documents without the need of a visual editor

I **strongly** encourage you to change the source of the various parts to see what happens (the output will change as you type)

Basic styles
------------

With this markup you can obtain *simple emhpasis* (usually rendered in italic text), **strong emphasis** (usually rendered in bold text), `source code` text (usually rendered in monospaced text), or ~~strikethrough~~ text (usually rendered with a line through text).

You may use also _this_ or __this__ notation to emphatize text, and you can use all them _**`together`**_ (and you can mix `*` and `_` )

If you look at the source code you may note that
even 
if 
you 
break 
the 
lines,
the text is kept together
in a single paragraph

 Paragraphs are delimited by blank lines, leading and trailing spaces are removed 

You may force a line break with two spaces  
or with a `\`\
at the end of the line

Basic styles
------------

With this markup you can obtain *simple emhpasis* (usually rendered in italic text), **strong emphasis** (usually rendered in bold text), `source code` text (usually rendered in monospaced text), or ~~strikethrough~~ text (usually rendered with a line through text).

You may use also _this_ or __this__ notation to emphatize text, and you can use all them _**`together`**_ (and you can mix `*` and `_` )

If you look at the source code you may note that
even 
if 
you 
break 
the 
lines,
the text is kept together
in a single paragraph

 Paragraphs are delimited by blank lines, leading and trailing spaces are removed 

You may force a line break with two spaces  
or with a `\`\
at the end of the line

Images
------

Syntax for images is like the syntax for links, but with a `!` before:

![alt text](https://raw.githubusercontent.com/agea/tutorial.md/master/img/1.png "image title")

![](https://raw.githubusercontent.com/agea/tutorial.md/master/img/2.png)

![ref]

[ref]: https://raw.githubusercontent.com/agea/tutorial.md/master/img/3.png

Lists
-----

To define a list of items, just put a `*`, a `-`, or a `+` at the start of the line of each item of the list followed by at least a space, to end the list, leave a blank line

* red
* green
* blue

- white
- grey
- black
+ yellow
+ pink
+ orange

You can also define numbered list, putting a number followed by a `.` or a `)` and a space at the start of the line (you may use any number, the first one is taken to start counting, then it will increment by one):
 
3.
2. you may leave blank items
1) or start
1) again

You can insert any block inside a list, you have to respect the indentation of the text of the list item

- A *paragraph* of text
  (spanning multiple lines),
  
  ```
  fenced code,
  ```
   
      indented code (4 spaces + 2 spaces for the list 
      indentation, one blank line above, one below),

  > quotes,
   
  - another 
    * list
      + (and so on...),
      
  - ### or headers
  
Headers
-------

There are two ways to define headers:

The biggest possible header
===========================

# You can also use this markup

(I prefer the first one as it's more readable when looking directly at the source code)
 
A sub heading
-------------
 
## This is the alternative format

### Then you can go smaller

#### And smaller

##### And even smaller

###### No, you can't go smaller than this

The good thing is that many tools that convert Markdown in HTML or PDF are able to generate the index of your document, or links to the headers automatically (like Github does on the [source](http://git.io/vfz98) of Markdown files)

Horizontal rules
----------------

You can use horizontal rules to separate paragraphs: you may use three or more `*`
******
or three or more `_` (you may insert spaces before, after or between the characters, no other charachters are allowed)
__ __ __ __ 

or three (or more) `-` 

---

but you have to be careful as it is similar to the header syntax, so if you write `---` immediatly after a single line of text you get an header, either you have to leave a blank line before the `---`, or you put it after multiple lines of text

Source code
-----------
If you have to insert code in your document you have three choices:
 1. inline code like this: `*Hello* **world!**"`
 2. fenced code blocks (you may use ` ``` ` or `~~~` 
   as delimiters): 
``` markdown
*Hello* **world!**
```
 3. indented code blocks
    *Hello* 
    
    **world!**
You need to leave a blank linke after a paragraph and 2 blank lines after a list to start an indented code block (if you want the code to beout of the list), and you may insert the name of the language immediatly after the opening code fence (so some renderers may be able tohighlight the syntax of the language)
Blockquotes
-----------
> In this way you define a quoted block of text.
You can skip the initial `>` in intermediate lines 
if you are in the same paragraph
>
>> (you may nest levels)
    > but you can't indent with more than 3 spaces


Entities
--------

With the goal of making Markdown as HTML-agnostic as possible, all [valid HTML entities][] are recognized and converted into unicode characters

Named entities consist of `&` + any of the valid HTML5 entity names + `;` 

Some examples:

- `&amp;` &amp; 
- `&copy;` &copy;
- `&rarr;` &rarr;

[valid HTML entities]:https://html.spec.whatwg.org/multipage/entities.json

Escaping
--------

If you have to write something that would result in a Markdown vaild syntax, you can escape the first character of your expression (you may also escape any other punctuation character) with a `\`

\*not emphasized\*

\<br/> not a tag

\[not a link](/foo)

\`not code`

1\. not a list

\* not a list

\# not a header

\[foo]: /url "not a reference"

You may also escape the backslash itself \\*like this*

About
-----

You can read the [Wikipedia](http://en.wikipedia.org/wiki/Markdown) article, if you want more information about Markdown.

There are several Markdown *flavours*, this tutorial is based on the [Commonmark](http://commonmark.org) specification.

You may want to read the full [Commonmark specification](http://spec.commonmark.org/)

The source code for this tutorial is on [Github](https://github.com/agea/tutorial.md)

### License

MIT - <http://www.opensource.org/licenses/mit-license.php>
