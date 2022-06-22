# Gramatik

Work with Chomsky grammers.

#### Example

```rust
let gramatik = Gramatik(
	['A', 'B', 'S'].into(),
	['a', 'b'].into(),
	ruleset!(
		S -> _
		A -> a | aB
		B -> b | _
	),
	'S'
);

println!("The gramatik's class: {}", gramatik.class());

if gramatik.is_class(2) {
	println!("The gramatik is class 2!");
}
```

Based on the lectures of Krisztian Tichler Dr.

#### Disclaimer

This is only for fun and learning, I failed the exam twice already. Do not trust this code.

###### _(c) 2022_
