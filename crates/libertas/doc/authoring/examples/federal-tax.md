<h1 style="font-size:2.6rem;line-height:1.1;">Federal tax return App</h1>

**A guided tax interview with source-based calculations and on-screen results.**

The `libertas-tax` prototype collects household information, income,
deductions, credits, and payments, then calculates a U.S. federal tax year 2026
estimate. It uses synthetic data and does not file a return. Printable forms
and electronic submission are not part of the prototype.

## What the App does

- **Asks, then calculates.** The interview records the facts a return is made
  of: who is filing, the people they support, each income document, and the
  adjustments, deductions, credits and payments that apply. Every calculation
  starts from those recorded facts.
- **Shows the return on screen.** The estimate presents Form 1040 line by
  line with its supporting schedules, worksheets and credit forms, so each
  amount can be traced to what was entered.
- **Estimates, and says so.** Ordinary income tax uses the
  [draft 2026 Tax Table](https://www.irs.gov/pub/irs-dft/p1040--dft.pdf) below
  $100,000 and its Tax Computation Worksheet above it; the earned income credit
  uses the [draft 2026 EIC Table](https://www.irs.gov/pub/irs-dft/p1040--dft.pdf).
  Until the final 2026 forms and tables are published, the result is an
  estimate, not a filing-ready return.
- **Refuses rather than guesses.** A situation the App does not calculate is
  reported as unsupported with the reason. It never silently contributes zero
  tax or a partial amount.

## How the interview works

The interview is a Libertas workflow. Each page is one request; the App decides
what page follows from the answers, and every accepted page is saved on the
Hub, so the return can be reopened later from any device. Large pages can
restore field edits that were completed on the same device; unfinished typing
is discarded.

Each person's return is their own. Every request carries the signed-in user's
identity in a hidden field that the Hub verifies against the session before
the App sees it, and the App keeps each user's return in that user's own
database, so two people using the same task never see or touch each other's
answers.

The sections, in the order a filer meets them:

1. **Your 2026 return** — return identity, filing status and the basic
   questions that shape everything after.
2. **About you** and, for a joint return, **About your spouse** — identity,
   age, residency and dependency information.
3. **Children and people you support** — each dependent, with the tests that
   decide who may claim them and which credits they qualify for.
4. **Your 2026 at a glance** — a checklist choosing the income, deduction,
   credit and other tax situations in the return.
5. **Income** — each W-2, 1099, K-1, retirement, farm, business or other
   income record, entered as the document presents it.
6. **Adjustments to income** — IRA, HSA, self-employment, student loan and
   the other Schedule 1 adjustments.
7. **Deductions** — the standard deduction or itemized deductions, with their
   limits and carryovers.
8. **Credits** — child, dependent care, education, retirement savings, energy,
   vehicle, foreign tax and business credits.
9. **Other taxes** — alternative minimum tax, net investment income tax,
   additional Medicare tax and additional retirement or account taxes.
10. **Federal payments** — withholding, estimated payments and underpayment
    information.
11. **Anything else?** — the less common whole-return situations, so an
    ordinary filer is not asked about them earlier.
12. **Review** (*Review your federal estimate*) — the on-screen Form 1040, the
    amount owed or refunded, and the list of anything unsupported or still
    unanswered.

Each section hub offers its own **Continue** action while walkthrough steps
remain. It records completion of that section and opens the next applicable
section, which can be Review. Once the walkthrough is complete, the task
offers **Review** instead, opening the main information hub. Accepted edits
that reopen walkthrough work can restore Continue. The
[task-controlled caption pattern](../reference/presentation.md#task-controlled-section-hub-action)
keeps the button's name and operation aligned. Topic links and a previous-section
link remain on each hub; the redundant next-step row is removed.

The Income, Adjustments, Itemized deductions and Credits hubs, plus Review,
declare a secondary **Go to** control and **AI Search** across the return's
available topics. Eight groups organize the existing 46 destination identities;
real prerequisites still decide which pages are offered. Opening a page leaves
unfinished sections unfinished. Continue and each editor's submission retain
their separate responsibilities.

Search uses actual topic links and the same eligible destinations as Go to;
there is no repeated All topics catalog. A matched Go to destination opens
directly without reopening its chooser.
Descriptions recognize formal references and common names; the shared page
prompt also recognizes bare identifiers case-insensitively and prioritizes exact
matches. For example, Section 199A, `199a`, Form 8995, `8995`, Form 8995-A,
qualified business income and QBI deduction identify the Self-employment entry;
an explicit rental or K-1 question identifies its more specific supporting page.
Ambiguous queries can return alternatives, and unrelated queries can return no
match. These references identify pages and do not establish tax eligibility.
Live model ranking has not been evaluated for this prototype.

Grouped Go to uses the
[general hierarchical builder](../reference/protocols.md#hierarchical-single-field-requests).
The suggested category seeds the initial selection; choosing another category
preserves that explicit choice. Studio and web require a build with the corrected
[Union construction rule](../reference/schema.md#shared-copyfrom-and-copyto-rules);
earlier builds may reapply the initial category while switching groups.

Interview editors use **Save and continue**, **Previous step**
where a previous action exists, and **Close editor** when embedded beside a hub.
The final confirmation uses **Finish review** and **Return to review**. These
captions describe the controls; they do not file
a return. Clients that do not support button captions keep their system labels.

In a full-page workflow, **Previous step** reopens the declared previous page.
On a tablet or desktop, the Income, Adjustments, Itemized deductions and
Credits hubs use two panels, and the
[pane contract](../reference/presentation.md#master-detail-panes) gives the
left panel all navigation: select one of its section topics to open it on the
right; Go to and Search use the leading page's context and ordinary action
routing. Use **Save and continue** to submit
the right-panel edit, or **Close editor** to dismiss it without submission.
The right panel offers Close on every page it shows, never Back, Cancel or
Previous step. The left panel's Continue and Review actions, and links outside
the detail nodes such as the previous-section link, replace both panels; they
do not submit the right-panel editor. While either panel waits for a reply,
neither panel can send another request.
The [heading contract](../reference/presentation.md#protocol-page-heading-and-top-bar)
also assigns the shared top-bar title and Actions to the left page. It requires
the full title to remain in content whenever the top-bar title is shortened;
the right page keeps its own heading and Actions in its panel.
Pages are validated on the device before they are sent; a rejected page keeps
the editor open with the
problem shown. Two people opening the same return see the same accepted
answers, and an older editor cannot overwrite a newer accepted page.

## Worked examples

One example per area. Each is a synthetic case the App's own tests reproduce.

**A combined ordinary return.** A filer with W-2 wages, an IRA contribution,
FLSA overtime, a vehicle-interest deduction and education expenses reaches an
adjusted gross income of $72,500, taxable income of $49,400, income tax before
credits of $5,683, final tax of $4,183 and a refund of $3,317.

**Tips and overtime (Schedule 1-A).** A sole proprietor with a $10,000 profit
that includes $10,000 of qualified customer tips deducts $9,293 of tips after
the $707 half-self-employment-tax deduction. The tip deduction shares the
$25,000 cap and income phaseout with employee tips and does not reduce business
profit or self-employment tax. See the
[draft 2026 Schedule 1-A](https://www.irs.gov/pub/irs-dft/f1040s1a--dft.pdf).

**Casualty losses (Form 4684).** Two federally declared disaster events with
$30,000 and $10,000 of unreimbursed personal-property loss, at $60,000 of
adjusted gross income, yield $33,800 of loss before the itemized-deduction
limit: $100 is subtracted per event, then 10% of AGI from the combined loss.

**Charitable contributions (Schedule A and Form 8283).** A volunteer's 100
service miles plus $6 of parking contribute $20 at the 14-cent statutory rate.
Alongside a $1,000 cash gift at $60,000 AGI, itemizing deducts $720 after the
$300 charitable floor. Cash gifts above their 60%-of-AGI room carry forward for
five years, and property gifts follow
[Publication 526](https://www.irs.gov/publications/p526).

**Retirement distributions (Form 1099-R).** Employer stock distributed at
$50,000 and sold for $40,000, with $25,000 rolled into an eligible account,
produces $18,750 of pension income and a $3,750 capital loss before the annual
capital-loss limit. Rolling over all proceeds recognizes neither gain nor loss.
See [Publication 575](https://www.irs.gov/publications/p575).

**Annuities (the General Rule).** An original $100 monthly payment with a 45%
exclusion ratio recovers $675 of cost over 15 delayed installments, unless the
remaining investment in the contract is lower. See
[Publication 939](https://www.irs.gov/publications/p939).

**Foreign earned income and housing (Form 2555).** $50,000 of foreign housing
expenses at the 2026 general limit produce $18,606 before allowances; a $5,000
tax-free government housing allowance reduces the exclusion to $13,606. A filer
with $50,000 of profit in each of a foreign and a U.S. business owes $14,130 of
self-employment tax; the half-SE deduction is allocated once between them, so
the net foreign exclusion is $46,468 and AGI is $46,467. See
[Publication 54](https://www.irs.gov/publications/p54).

**Installment elections for capital gains.** A supported $40,000 capital-gain
sale produces $6,000 of attributable tax, of which $1,500 is due initially and
$4,500 is deferred; the return's other tax remains due normally.

**Civil-service death benefits (Publication 721).** A $38,400 lump sum paid to a
beneficiary, with $1,000 previously recovered against $45,000 of employee cost,
produces no taxable amount and identifies the cost still unrecovered.

## Coverage

The App targets the forms and schedules a broad consumer federal return uses.
It calculates the common ones completely and reports the others as
unsupported. The lists below name forms; the interview asks in plain language
and maps the answers to them.

**Calculated:** Form 1040 with Schedules 3, A, C, D, EIC, H, R and 8812;
income documents W-2, W-2G, 1098-E, 1098-T, 1099-OID, 1099-R, 1099-SA and
1099-B with Form 8949; Form 2210 underpayment penalties, including the
annualized-income method, and the next-year Form 1040-ES estimated-tax
computation; and Forms 2439, 2441, 2555, 4136, 4137, 4952, 5695, 6252, 7203,
8379, 8396, 8582, 8606, 8615, 8829 (including the actual-expense method),
8834, 8839, 8853, 8859, 8862, 8863, 8880, 8881, 8889, 8915-F, 8936 and 8959.

**Calculated for the common situations, with named exceptions:** Schedules 1,
2, B, E (page 1), F, J and SE; income documents 1099-INT, 1099-DIV, 1099-G,
1099-K, 1099-NEC, 1099-MISC, 1099-Q and 1098; Schedules K-1 from partnerships,
S corporations and estates or trusts; Forms 1116, 3800, 4562, 4684, 4797,
5329, 6198, 6251, 7206, 8283, 8801, 8911, 8941, 8958, 8960, 8962, 8995 and
8995-A; Form 982 debt cancellation; and the Form 8938 filing-requirement check. The interview names the exception when one
applies.

**Not yet calculated:** Forms 2106, 3903, 4972, 8814, 8815, 8332 and 2120;
Forms 4835, 6781, 8824, 8857 and the Schedules K-3; and the investment,
housing, alternative-vehicle and paid-leave credits (Forms 3468, 4255, 8586,
8910 and 8994). Nonresident returns
(Form 1040-NR) and foreign partnership reporting (Form 8865) are outside the
prototype's scope.

**Deferred with filing:** vouchers, extensions, amended returns, direct deposit,
identity-theft affidavits, Form 1310 and the other filing-administration forms.

## Sources and limits

Tax rules are taken from IRS forms, instructions and publications for tax year
2026, linked from the interview where a rule applies. Money is exact to the cent
and rounded where the forms round. The App verifies neither documents nor
identities, and it does not check dependency claims, qualifying expenses or
institution details with third parties: it calculates from the facts entered,
and asks the filer to confirm eligibility where the law requires it.
