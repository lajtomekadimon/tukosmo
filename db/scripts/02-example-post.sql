
DO

LANGUAGE PLPGSQL

$$

DECLARE

    lang_en BIGINT;
    lang_es BIGINT;

    user_id BIGINT;

    file_id BIGINT;

    post_id BIGINT;
    post_title TEXT;
    post_description TEXT;
    permalink_value TEXT;
    post_body TEXT;

BEGIN

    user_id := 1;

    file_id := i_file(
        'featured-image-default-post.jpg',
        user_id
    );

    PERFORM u_file(
        file_id,
        'Ocean With Rock Formation Under Starry Night'
    );

    post_id := i_post(
        file_id,
        user_id
    );

    post_title := 'Example post';
    post_description :=
        'In this post, we will test MarkDown (CommonMark).';
    permalink_value := 'example-post';
    


    post_body :=
'# h1 Heading
## h2 Heading
### h3 Heading
#### h4 Heading
##### h5 Heading
###### h6 Heading


## Paragraphs

This is a paragraph. You can use bold text like **this** or __this__. You can also use italic text like *this* or _this_. You can even use automatic quotations[^a]. It''s so cool!

[^a]: Random text.

## Horizontal Rules

___

---

***


## Blockquotes

> Blockquotes can also be nested...
>> ...by using additional greater-than signs right next to each other...
> > > ...or with spaces between arrows.


## Lists

Unordered:

+ Create a list by starting a line with `+`, `-`, or `*`
+ Sub-lists are made by indenting 2 spaces:
  - Marker character change forces new list start:
    * Ac tristique libero volutpat at
    + Facilisis in pretium nisl aliquet
    - Nulla volutpat aliquam velit
+ Very easy!

Ordered:

1. Lorem ipsum dolor sit amet
2. Consectetur adipiscing elit
3. Integer molestie lorem at massa

Task list:

- [ ] Bla
- [x] Ble
- [x] Bli
- [ ] Blo
- [x] Blu


## Tables

Left aligned columns:

| Option | Description |
| ------ | ----------- |
| bla    | bla bla     |
| bla    | bla bla     |
| bla    | bla bla     |

Right aligned columns:

| Option | Description |
| ------:| -----------:|
| bla    | bla bla     |
| bla    | bla bla     |
| bla    | bla bla     |


## Code

Inline `code`.

Indented code:

    // Some comments
    line 1 of code
    line 2 of code
    line 3 of code


Block code "fences":

```
Sample text here...
```

Syntax highlighting

```js
var foo = function (bar) {
    return bar++;
};

console.log(foo(5));
```


## Links

[link text](https://wikipedia.org)


## Images

![Tukosmo](/static/img/tukosmo-logo-128.png)';



    PERFORM i_post_translation(
        post_id,
        s_language_id_by_code('en'),
        post_title,
        post_description,
        post_body,
        permalink_value,
        1,  -- user ID
        FALSE
    );

END;

$$;
