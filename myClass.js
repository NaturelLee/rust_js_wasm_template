export class MyClass {
  constructor() {
    this.num = 78;
  }

  get number() {
    return this.num;
  }

  set number(n) {
    return (this.num = n);
  }

  render() {
    return `the number is ${this.num}`;
  }
}
