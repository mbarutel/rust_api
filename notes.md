# Notes to self

So, the registration handler and service for a delegate registration is set, albiet, it still needs more work.
Here are some things that still need thinking about:

- [ ] What should be the logic for pricetiers? AI is suggesting that it be persisted?
  - My original idea is to auto calulate it in the form rego. But it may cause undesirable behaviour when the start date of the conference changes. Alternatively, we auto calculate it an conference creation and persist it. When the start date changes, the admin may then choose to re-calculate the price tiers or keep the original one.
- [ ] promocodes should be persisted in the backend. The only things to think about is how to implement the Buy X pay for X tickets? AI should be able to help with that.
 
