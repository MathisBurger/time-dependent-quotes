<div align="center">
    <h1>time-dependent-quotes</h1>
<hr>
<strong>Create time dependent quotes you can use to proof your text.</strong>
    <br>
<img src="https://img.shields.io/github/license/mathisburger/time-dependent-quotes?style=for-the-badge" />
<img src="https://img.shields.io/github/last-commit/mathisburger/time-dependent-quotes?style=for-the-badge" />
<img src="https://img.shields.io/github/v/release/mathisburger/time-dependent-quotes?style=for-the-badge">
</div>
<hr>
<div align="center">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/2048px-Rust_programming_language_black_logo.svg.png" height="150" />
</div>

# Project information

The project was an idea I had in an english exam. The quotes in the text had a timestamp, but how 
can you prove that this is really the text of the source at that specific time.
Therefore, I started to create my project `time-dependent-quotes`. It makes it possible to 
upload a file which creates a new quote. You can give this quote a title. Furthermore, the content of the file
is hashed so you can prove that this source contains exactly the provided content. The source is also saved with a UNIX timestamp 
that proves that the quote is created at that time. 

# User interface

The user interface looks quite ugly like any other university application. 
It has no extra css styling and only uses plain html elements that are rendered in the web.
But it is superfast because of the template rendering engine in the backend

# Techstack

This application is completely built with rust. Therefore, it is amazingly fast and the
application is actually very performant. It relies on a postgres database which is used to
save the quote related data. But we do not use any fancy web framework for our application, because they 
might have an impact on our performance. We render the whole content in the backend with our fast rust
web template rendering engine.

# Installation

You can just use our docker image to host the application. 
But make sure you can connect to postgres and create a volume to save 
the data that comes through the quotes. 

# Environment variables

If you are using docker to setup the application you also have to 
provide a `DATABASE_URL` environment variable that contains a database url to
your postgres database. 
