# Leptos Issue with Signals on Cleanup

When reading signals within on_cleanup (not the untracked methods), it can cause an error/warning in the console.

```
At src/main.rs:52:38, you access a signal or memo (defined at src/main.rs:50:39) outside a reactive tracking context. This might mean your app is not responding to changes in signal values in the way you expect.

Here’s how to fix it:

1. If this is inside a `view!` macro, make sure you are passing a function, not a value.
  ❌ NO  <p>{x.get() * 2}</p>
  ✅ YES <p>{move || x.get() * 2}</p>

2. If it’s in the body of a component, try wrapping this access in a closure: 
  ❌ NO  let y = x.get() * 2
  ✅ YES let y = move || x.get() * 2.

3. If you’re *trying* to access the value without tracking, use `.get_untracked()` or `.with_untracked()` instead.
```

The solution is to use `.get_untracked()` instead of `.get()` in this case, though there's a couple of issues with it as is:

1. The warning in the console doesn't point to the right place in the file.
   I couldn't replicate this in this repo, but in my real project it was simply pointing to the line that the signal was defined in.

2. It would be nice if leptos could understand that we're in a cleanup context and not show any errors for this.
