import { Component } from '@angular/core';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.less']
})
export class AppComponent {

  signature: string;

  constructor() {
    this.signature = `Józef Podlecki ${new Date().getFullYear()}`
  }
}
