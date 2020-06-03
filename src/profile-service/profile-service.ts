import { faBomb, faFlag, faSyncAlt, faSmile, faFrown, faGlobe } from '@fortawesome/free-solid-svg-icons';
import { faLinkedin, faStackOverflow, faGithub } from '@fortawesome/free-brands-svg-icons';
import { Inject } from "@angular/core";
import { of } from 'rxjs';
import { Profile } from 'src/models/Profile';

@Inject({
    providedIn: 'root'
})
export class ProfileService {
    constructor() {
        
    }

    getProfile() {

        const signature = `Józef Podlecki ${new Date().getFullYear()}`;
        const social = [
            {
                href: "https://jozefpodlecki.github.io/",
                icon: faGlobe
            },
            {
                href: "https://stackoverflow.com/users/2304474/j%c3%b3zef-podlecki?tab=profile",
                icon: faStackOverflow
            },
            {
                href: "https://github.com/Jozefpodlecki",
                icon: faGithub
            },
            {
                href: "https://www.linkedin.com/in/jozef-witold-podlecki",
                icon: faLinkedin
            }
        ];

        return of<Profile>({
            signature,
            social
        });
    }
}