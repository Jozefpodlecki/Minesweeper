import { Component, OnInit } from '@angular/core';
import { ProfileService } from 'src/profile-service/profile-service';
import { Social } from 'src/models/Social';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.less']
})
export class AppComponent implements OnInit {

  signature: string;
  social: Social[];

  constructor(private _profileService: ProfileService) {
    this.signature = ''
    this.social = []
  }

  ngOnInit(): void {
    this._profileService.getProfile()
      .subscribe(({signature, social}) => {
        this.signature = signature;
        this.social = social;
      })
  }

  
}
