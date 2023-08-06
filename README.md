# kemuler-windows
Simulate input on Windows using [`kemuler`](1).

An improvement over [`enigo`](2)'s Windows implementation.

<details>
  <summary>Differences to <code>enigo</code></summary>
  Mouse input will work on application (mostly games)
  that uses DirectX/DirectInput/<i>somethingsomething</i>;
  
  that is currently not the case on <code>enigo</code>,
  see this [issue](3).

  <code>VirtualKey</code> enum's variants are more easier to search for.
  They had more intuitive names and doc aliases are added.
  More detailed documentation is also written.
</details>

[1]: https://github.com/kemuler/kemuler/ "kemuler Repository"
[2]: https://github.com/enigo-rs/enigo/ "enigo Repository"
[3]: https://github.com/enigo-rs/enigo/issues/172/ "enigo's issue"
