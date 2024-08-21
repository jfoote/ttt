# ttt

Trusted Types Test

[Try it](https://ttt.foote.dev).

Simple demo of [Trusted Types Content Security Policies](https://developer.mozilla.org/en-US/docs/Web/API/Trusted_Types_API). Made for fun when I was reading about them.

This is a [Fastly Compute](https://www.fastly.com/products/edge-compute) service. Serve it at `localhost:7676` with the [Fastly CLI](https://github.com/fastly/cli) like this:

```
$ fastly compute build
$ fastly compute serve
```

The web service (`src/main.rs`) returns an index page (`src/index.html`). The index page includes a "fiddle" (`src/fiddle.html`) in an iframe. Different buttons on the index page route to different paths in the web service, causing it to return different Trusted Types Content Security Policies.
