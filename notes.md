# Notes to self

## Business logic

- [ ] PriceTiers will have their own table and point to a conference. Will generate and save at conference creation and on manual trigger.
  - [ ] price_tier_repository implement
  - [ ] conference service to generate and save price tiers at creation
  - [ ] create endpoint on conference for re-generating price tiers
- [ ] promocodes should be persisted in the backend. The only things to think about is how to implement the Buy X pay for X tickets? AI should be able to help with that.
- [ ] need to complete get_conference_form data services, only the trait is done in the application layer
 

## API Structure
- [ ] move trait of repository/ from application. it should just all be in the infrastructure/ layer
- [ ] API versioning
- [ ] Redis setup
