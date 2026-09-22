<h1 style="font-size:2.6rem;line-height:1.1;">Federal tax return App</h1>

**A guided tax interview with source-based calculations and on-screen results.**

The `libertas-tax` prototype collects household information, income,
deductions, credits, and payments, then calculates a U.S. federal tax year 2026
estimate. It uses synthetic data and does not submit a return.

This App prepares individual-return estimates, including Schedule C sole-proprietor
businesses and ordinary cash- or accrual-method Schedule F farms. Corporate and partnership entity returns are outside its scope;
ordinary U.S. S-corporation K-1 businesses are supported when their required
records are complete. Ordinary partnership Form 1065 records can now be saved,
but partnership return calculations remain unfinished and prevent a complete
estimate. Estate/trust Form 1041 K-1s remain unsupported. Do not enter either
as S-corporation income; identify unentered items in the uncovered-income check.

Native phone and web interfaces present the same guided interview, with
automated calculations using the App's implemented 2026 federal tax rules.
Coverage includes foreign employment and supported foreign tax credits,
residential rental depreciation and passive-loss limits, AMT and ordinary early
retirement taxes, eligible rollovers, capital-loss carryovers and the
high-income itemized limitation. Schedule C calculations include cost-valued
inventory, eligible start-up costs, simplified home-office deductions, meal-expense limits and standard
vehicle mileage using both 2026 rates. Qualifying purchased equipment placed in
service in 2026 can use 100% bonus depreciation when acquired after January 19,
2025, or 20% for qualifying acquisitions from September 28, 2017 through January
19, 2025. The App allocates documented business use, then calculates GDS on any
remaining basis. The deduction can create a business loss; acquisition basis remains available
for QBI property limits. The de minimis annual election also expenses eligible low-cost purchases using
full invoice/item costs, financial-statement status and confirmed book-expensing
policy. Results show the election-statement requirement; see the
[IRS safe-harbor guidance](https://www.irs.gov/businesses/small-businesses-self-employed/tangible-property-final-regulations).
First-year GDS calculations also support class-wide bonus election out, ordinary
declining balance, optional 150% declining balance and straight line for eligible
3-, 5-, 7-, 10-, 15- and 20-year equipment. The return-wide purchase mix determines
half-year versus mid-quarter treatment, including fully bonus-deducted assets.
Results separately show bonus, MACRS and remaining adjusted basis. New assets
can use manual calculations or IRS percentage tables. Continuing schedules
preserve the original approach and convention; table rates apply to original
basis after original section 179 and bonus deductions, while manual calculations apply the required straight-line
switch. All six equipment classes and five conventions have IRS table coverage
for ordinary GDS, 150% declining balance and straight line. Start-up costs use
the phased initial deduction and 180-month amortization from the opening month,
or preserve the original capitalization election. Eligible section 197 acquired
intangibles use 180-month amortization from the later acquisition/business-start
month, with current deductions and remaining basis shown separately. Intangible
costs do not create QBI tangible-property basis. Later documented acquisition costs
use the remaining original months, or an immediate deduction after the original
period expires. Section 179 elections on eligible purchased equipment now apply the shared
2026 dollar limit and purchase phaseout, then the active-business income limit
including employee wages. Results allocate deductions and unused carryforwards
by asset, using current elections first and prior carryforwards oldest year first.
Separate filers supply the spouse's eligible purchases and elected shares; the
App combines purchases before applying the shared phaseout, then allocates the
dollar limit. Equal shares apply when no different allocation is elected or the
percentages do not total 100%. Continuing schedules keep unused elected cost outside depreciation
basis, while the home-office calculation follows the required deduction order. Transfer adjustments, basis decreases, unresolved contingent-debt
allocation/timing and dispositions remain unfinished; see
[Form 4562 instructions](https://www.irs.gov/instructions/i4562). Section 179 owner health/retirement and special
benefit-income combinations, ADS,
listed property/vehicles, changed use and dispositions remain unfinished. See
[IRS Notice 2026-11](https://www.irs.gov/irb/2026-06_IRB) and
[Revenue Procedure 2026-15, section 2.02](https://www.irs.gov/irb/2026-13_IRB). Ordinary self-employed health premiums and
owner-only traditional SEP contributions coordinate with income, QBI, IRA limits
and medical itemization. Coverage for a nondependent child under 27 includes
the incremental-premium allocation: unused child-only premiums never become
a Schedule A deduction. See [IRS Publication 502](https://www.irs.gov/publications/p502). SEP plans use the entered employer contribution rate
(in hundredths of one percent), its self-employed adjustment, the $360,000
compensation ceiling and the $72,000 annual contribution cap for 2026. See
[the IRS self-employed worksheet](https://www.irs.gov/publications/p560) and
[2026 plan limits](https://www.irs.gov/retirement-plans/cola-increases-for-dollar-limitations-on-benefits-and-contributions). Qualified long-term-care premiums share the 2026 per-person age cap
across personal, business and public-safety policies. Tax-free public-safety
payments are removed first; eligible unused premiums enter medical itemization
once. Employer long-term-care eligibility is recorded separately from
ordinary health coverage. See [the 2026 age limits](https://www.irs.gov/irb/2025-45_IRB).
Long-term-care benefit records also calculate qualified periodic payments and
chronic-illness accelerated death benefits using the 2026 $430 daily limit,
qualified care costs, reimbursements and shared recipient allocations. Qualified
reimbursement and terminal-illness exclusions have separate source paths. See
[Form 8853 instructions](https://www.irs.gov/instructions/i8853). Medicare Advantage MSA annual withdrawals now use
qualified medical spending, payment dates, disability onset, prior-year account
value and the January 1 deductible to calculate taxable income and the separate
50% additional tax. Spouses are calculated separately; age 65 is not an exception,
and excluded medical payments are not also deducted. Inherited HSAs and both MSA
types now support 2026 death-date values, a nonspouse beneficiary's qualifying
medical-payment offset, estate-beneficiary inclusion on the holder's final return,
and tax-free continuation by a designated spouse. Separate accounts retain separate
calculations and no death-transfer penalty. Earlier-death records preserve the
historical value without taxing it again in 2026; their medical offsets belong on
the death-year return, including any amendment. Documented post-death interest,
dividends and investment sales link to the beneficiary's ordinary income entries
and are calculated once with their own tax treatment. Unclassified 1099-SA
settlement earnings still require review. An inherited Medicare MSA uses Archer
distribution rules for a designated spouse but cannot receive new contributions. Ordinary Archer MSA withdrawals now exclude
qualified medical use and verified same-owner indirect rollovers, then apply the
20% additional tax with age/disability exceptions. Rollover dates enforce the
60-day deadline and preceding-year limit; spouses' taxable amounts combine before
applying the rate. HSA-to-HSA indirect rollovers likewise use the transfer dates
and prior-rollover history, remain excluded without consuming contribution
capacity, and retain separate records for each spouse. Direct trustee transfers
are omitted; Archer-to-HSA transfers are reported once at their source. Ineligible
transfers still require excess-contribution treatment. Qualified IRA-to-HSA
funding transfers made in 2025 now have separate testing-period recapture for
2026 eligibility loss, including the 10% tax and death/disability exceptions.
Current-year direct funding from a traditional IRA now links to
its source income entry: gross distributions and withholding remain visible,
while transferred dollars are excluded from IRA income and early-distribution
tax. Funding reduces other HSA contribution capacity without creating a deduction.
Coverage at transfer and age determine its maximum; a second transfer requires
the permitted later-month self-only-to-family change. Testing failures coordinate
with the last-month rule without recapturing the same non-IRA contribution twice.
When prior-year basis exists and funding precedes other current-year IRA activity,
the App uses filed prior basis and dated aggregate IRA values to consume taxable
money first. Basis used by funding is removed; retained basis remains available
for later withdrawals and conversions. QCD allocation excludes the funding dollars.
Roth-to-HSA funding also uses a linked dated Roth source, aggregate account value
and opening contribution/conversion history when funding precedes other current-year
Roth basis activity. It uses taxable earnings first, then consumes basis without
conversion recapture, before later contributions and conversions are added.
Ordinary withdrawals cannot reuse the consumed basis. Inherited traditional and
Roth sources are supported using the same beneficiary's HSA and the separate
basis of the linked deceased-owner group. The lifetime limit includes personal
and inherited transfers together; neither funding nor its consumed basis is
counted again in QCD or estate-tax deduction calculations. Earlier current-year
basis activity still needs chronological accounting. See
[Notice 2008-51](https://www.irs.gov/irb/2008-25_IRB). Supported last-month-rule paths
and their remaining allocation limits are described below.
Archer contribution deductions now use monthly HDHP records,
2026 deductible/out-of-pocket limits, linked employer or business compensation,
and family allocation agreements. Employer deposits are not deducted again and
can bar personal deposits under the family rules. Archer contributions also reduce
HSA contribution capacity, including shared-family coordination. Separate-return
Archer calculations use the other spouse's facts without deducting their deposits;
separate-return family HSA allocation supports either actual monthly eligibility
or the December method described below. Timely personal
HSA corrections now remove the returned principal from contributions and report
positive attributable earnings in the withdrawal year. The same treatment covers
corrected employer principal already included in W-2 wages, without counting it
twice. Corrections of 2025 employer principal reported as 2025 other income also
keep that principal out of 2026 income; only earnings returned in 2026 enter this
year. Remaining excess, late corrections, 2026 employer principal omitted from W-2 wages, and
Archer correction cases remain unfinished. Prior-year HSA last-month-rule
recapture now uses actual 2025 contributions and monthly coverage, with age-55
catch-up, Archer contribution offsets, and death/disability exceptions. Overlapping
IRA-funding transfers retain their individual testing periods; last-month income
is capped at non-IRA contributions, and both amounts share one 10% tax per
owner. For independent limits, 2026 contributions use the greater of the monthly or
December-based limit, with age-55 catch-up and Archer offsets. The result shows
the amount requiring continued eligibility through December 2027; a known
nonexcepted first loss in December 2026 enters the current return. Historical
family allocation now uses both spouses’ coverage, Archer deposits and
an agreed exact ratio, keeping catch-up allowances separate. Joint filers can also select the 2026 December family allowance with one
agreed division, using the existing HSA coverage records. The shared annual
cap and each owner’s testing-period income remain separate from their personal
catch-up. Separate filers can use the same December method with coverage-only
spouse context; the spouse’s return amounts are excluded. Historical switches
to self-only by December use the owner’s independent December allowance,
retaining a higher actual monthly allowance when applicable. Current joint and separate returns
can choose actual monthly family allocation instead, including when eligibility
ends before December; it prorates catch-up and adds independent self-only months
without using the extended testing period. Archer MSA, HSA and Medicare MSA
deemed distributions use January 1 account value for
prohibited-transaction terminations and dated collateral values for loan pledges,
with no medical or rollover exclusion. Age/disability exceptions use the deemed
distribution date, and each spouse's HSA additional tax is rounded separately.
Medicare MSA deemed amounts share the owner's single prior-balance allowance;
its 50% tax has no age exception, and disability/death relief starts on the event
date. See [the Medicare MSA rules](https://uscode.house.gov/view.xhtml?edition=prelim&num=0&req=granuleid%3AUSC-prelim-title26-section138) and
[the 2026 plan limits](https://www.irs.gov/irb/2025-45_IRB). See [Form 8889](https://www.irs.gov/instructions/i8889)
and [account statement instructions](https://www.irs.gov/instructions/i1099sa).
These calculations use the continuing rules in the published 2025 Forms 8853/8889
instructions; final 2026 form reconciliation remains pending.
Marketplace business plans and other self-employed retirement arrangements
remain unfinished. Ordinary business losses, activity-level QBI limits, qualified REIT dividends,
separate business and REIT/PTP loss carryforwards,
monthly HSA eligibility, nonmedical HSA withdrawals and adoption credits with
five-year carryforwards also flow into the on-screen return. Ordinary Marketplace
premium tax credit reconciliation includes additional credits, excess-advance
repayment and medical-deduction coordination. Repayment also enters the
regular-tax liability used for the AMT comparison and foreign-tax credit limits,
following [Form 6251](https://www.irs.gov/pub/irs-pdf/f6251.pdf) and
[Form 1116 line 20](https://www.irs.gov/instructions/i1116). Ordinary Form 1116 also supports
the enhanced senior deduction: it reduces actual income tax but is excluded from
foreign expense allocation and the worldwide-income denominator. The denominator
is computed before the return's zero floor, including when the senior deduction
reduces actual taxable income to zero. See Form 1116 lines 3b and 18 in the same
instructions. Allowed traditional IRA deductions also feed this worksheet: expense
allocation uses gross income before the IRA deduction, and the AMT calculation
retains the IRA deduction while excluding the standard deduction. Supported
itemizing now allocates medical expenses, general sales taxes and personal-property
taxes after their deduction limits; AMT retains medical expenses and removes
those taxes. The existing automatic deduction comparison can choose itemizing
even below the standard deduction when it lowers total tax. An explicit choice
is preserved. Qualified home mortgage interest follows gross-income allocation,
with an explicit optional all-U.S. allocation when combined foreign gross income
is at most $5,000. Eligible U.S. cash gifts are allocated to domestic income,
including the non-itemizer deduction; they do not reduce foreign income. See
[Form 1116 line 4a](https://www.irs.gov/instructions/i1116) and the
[charitable-allocation rule](https://www.irs.gov/irb/2005-33_IRB).
Itemized state/local income taxes use records of each payment and its underlying
domestic and foreign income. The App reconciles payments, applies the SALT and
high-income deduction limits, then allocates to the existing foreign-source
groups. A state that exempts foreign income can have an entirely domestic share;
AMT removes the state taxes. The optional $5,000 all-U.S. interest allocation
also supports investment interest, using the existing independent regular and
AMT Form 4952 deductions. Ordinary portfolio investments also support asset-based allocation from
beginning/end adjusted bases or complete dated basis histories when endpoint
averaging distorts holdings. The App calculates daily weights independently for
regular tax and AMT. Current-year
asset values allocate both current interest and prior disallowed interest now
deductible. A mixed-source asset uses its recorded domestic and foreign gross
income to split its basis before allocating interest; source yields must
reconcile with entered income. Special valuation, unresolved or zero mixed-asset
yields, unresolved state attribution and other itemized classes still require
additional worksheets.
Direct foreign interest and ordinary dividends now include withholding-treatment
facts. The App groups these payments, reallocates a loss in one passive withholding
group proportionately to the other positive passive groups, then tests high-tax
treatment separately for regular tax and AMT. It shows the income and taxes
moved to the general category, including taxes on groups reduced to zero.
Remaining losses first offset the other
foreign category, then domestic income. Domestic losses reduce positive foreign
income proportionately. Results show new separate-limitation, overall-foreign
and overall-domestic loss account amounts for regular tax and normal AMT. Keep
these amounts for future recapture. Retained general/passive loss-account
balances now feed ordered recapture: overall foreign loss, separate limitation
loss, then overall domestic loss. Results show opposite-account netting,
recapture and closing balances independently for regular tax and normal AMT.
An optional additional overall-foreign-loss recapture election is checked
against available income and balances. Special dispositions, ownership changes,
enhanced pre-2018 domestic-loss elections and loss accounts with the simplified
AMT election still require additional worksheets.
This follows [the passive-income grouping regulation](https://www.law.cornell.edu/cfr/text/26/1.904-4)
and [Form 6251's AMT instructions](https://www.irs.gov/instructions/i6251).
The ordinary-source worksheet also supports a valid simplified AMT limitation
election made in the first eligible claim year and retained in later years.
It reuses regular net foreign income for the AMT limit; it can increase tax and
is not an annual choice of the cheaper method. The result shows the income used
for each category's AMT limit. Retained 2025 Schedule B balances now supply
prior-year carryovers: current taxes use the limit first, then the oldest balance
in each category, separately for regular tax and AMT. The App shows usage,
expiration and remaining balances, even with no current foreign income. Retained 2025 credit worksheets can also allocate new 2026 excess back one year,
separately by category and tax system, and show the remaining new carryforward.
A valid 2025 small-credit election prevents that carryback. If 2025 instead used
a foreign-tax deduction, retained hypothetical credit worksheets let the App
calculate mandatory absorption and the reduced carryforward. Absorption gives
no credit, deduction or refund while that choice remains unchanged; see
[Publication 514, deduction years](https://www.irs.gov/publications/p514).
Unreviewed histories remain visibly pending. Reconstructing missing prior-year
worksheets and computing prior-return refunds are not included. The carryback
allocation is not a refund amount.
See [Schedule B lines 6–7](https://www.irs.gov/instructions/i1116sb). Entity-specific
allocations, foreign losses and unresolved history adjustments remain outside
this worksheet. The AMT limit uses final AMT
income, including the high-income married-separate addition. Its expense
allocation excludes state-tax refunds removed from AMT, including when an IRA
deduction must be allocated.
Government unemployment entries now preserve gross benefits, actual withholding
and individual repayment records. Benefits received and repaid in 2026 are
netted once on Schedule 1 line 7; the repayment does not reduce withholding or
create a second deduction. The result shows gross, repaid and taxable amounts
for each source. Earlier-year repayments are combined across filers and sources,
including net Social Security repayments. When the supported total is $3,000 or
less, they give no current deduction or credit and do not reduce current benefits.
For larger governmental unemployment repayments, the comparison accepts original
2023–2025 ordinary-return income, filing status, fixed standard deduction and
filed income tax. It recomputes historical tax after removing the matching
repayments, then compares complete current returns using the deduction or credit.
Automatic selects the lower net tax; explicit method choices remain available.
The credit can exceed current tax. Childless earned-income credit is recomputed
from the original year's eligibility facts and published 2023–2025 tables,
including credits newly available after excluding repaid benefits. The result
shows original and recomputed tax and EIC separately. Current residency or SSN
answers never replace original-year facts. Age-based standard additions,
Schedule R elderly credit and the 2025 enhanced senior deduction are also
recomputed. Timely SSNs are entered once for both EIC and the senior deduction;
Schedule R uses the original year's qualifying nontaxable pension records when
they can affect the result. Regular-method Social Security is recomputed from
original SSA-1099/RRB-1099 net benefits and tax-exempt interest, carrying revised
AGI into the senior deduction and EIC and revised nontaxable benefits into
Schedule R. Separate returns use original-year living arrangements. Original-year
ordinary retirement records include fully taxable traditional IRA withdrawals
and employer-plan pensions with payer-determined taxable amounts. Their taxable income feeds the same historical calculations;
pension investment returned tax-free does not reduce Schedule R credit. See
[IRS pension reporting](https://www.irs.gov/instructions/i1040gi)
and [IRA distribution guidance](https://www.irs.gov/publications/p590b).
For early distributions, the original 10% or SIMPLE 25% tax is recomputed,
including the medical-expense exception above 7.5% of historical AGI. The App
reconciles original Form 5329 Part I tax, applies the medical floor once and
rounds each spouse's form separately. Any decrease joins the repayment credit;
unchanged additional tax creates no extra credit. See
[Form 5329 instructions](https://www.irs.gov/instructions/i5329).
Historical income is no longer limited to $100,000. The comparison switches
between each year's tax table and rate worksheet and recomputes AMT, including
specified private-activity-bond interest, exemption phaseout and separate-filer
adjustments. It adds back standard and enhanced senior deductions for AMT.
Original Form 1099-DIV records also support ordinary and qualified dividends,
capital-gain distributions and their 25%/28% components. Both tax methods
recompute preferential-rate bands. These sources enter Social Security's income
base and EIC's investment-income limit once. Historical capital records reuse
source proceeds, basis, holding period, wash-sale adjustments and personal-use
classification. Shared netting applies incoming short/long loss balances and
the $3,000 loss limit ($1,500 for separate filers), preserving special-rate
components. Original and revised regular/AMT balances are calculated separately. Changed
losses propagate through supplied consecutive historical returns, even when a
later year has no repayment. Enter actual incoming losses for reconciliation;
the App calculates revised balances and recomputes tax and EIC eligibility.
Records may be entered in any order. Complete original 2025 repayment worksheets
also supply current capital-loss inputs, without requiring the same worksheet
again. Separately entered current history must reconcile; unknown investment-loss
attribution still needs its source details. Balances differing entering 2026
are reconciled with these actual 2025 regular/AMT balances.
The App compares complete current returns without the repayment deduction or
credit, using actual versus refigured balances. The Chapter 1 tax decrease,
including affected refundable credits, is added once to the repayment credit;
actual current income and saved history remain unchanged. Selecting the credit
method reports the refigured remaining regular/AMT capital losses for future
years; selecting the deduction method retains ordinary balances. Missing
intervening years or unresolved current calculations prevent completion.
Historical sales accept separate AMT basis and wash-sale records. Incoming AMT
losses may differ from regular losses; an explicit zero differs from leaving
that option absent, which means equal balances. Both netting and preferential
rates use the applicable sources. When capital sources differ, retained permanent
capital-adjustment records distinguish exclusions from deferrals. An explicitly
empty list confirms no permanent component; an absent list does not. Fixed
source adjustments apply while incoming losses are unchanged. Loss differences
proven to arise from the calculated permanent deductions retain that origin;
other changing loss histories can supply an independent exclusion-only worksheet.
It retains separate incoming short/long losses and each sale’s differing basis
and wash-sale facts, reusing the proceeds and holding period. The App nets and
carries these losses separately in both comparisons and reconciles each subsequent
year. Use this complete worksheet or fixed capital adjustments, not both.
Historical comparisons can
use preceding-year Form 8801 source facts and incoming minimum-tax credits
with separate original and refigured balances. Supported unused credits flow
through consecutive supplied returns and the 2026 comparison. Complete original
2025 source facts also supply the current credit automatically; duplicate records
must reconcile even when the historical worksheet produces zero credit. A second
record cannot create a carryover or omit a known permanent adjustment. The calculation uses the preceding year’s rates, retains signed
credit generation, and limits use after other supported nonrefundable credits.
Propagation includes qualified dividends, capital-gain distributions and sales,
including 25% and 28% gain components, and newly generated deferral credits.
The carried worksheet preserves separate regular-tax band amounts and AMT gain
pools. Differing exclusion-only losses entering 2026, changed credit ownership,
and a changed credit balance still unused after 2026 require further calculation.
See [Form 8801](https://www.irs.gov/pub/irs-pdf/f8801.pdf). Special investment treatments
also need their own calculations. See
[Schedule D instructions](https://www.irs.gov/instructions/i1040sd).
NIIT is outside the Chapter 1 tax decrease used for this credit; a hypothetical
NIIT decrease is not added. See [Form 6251](https://www.irs.gov/instructions/i6251)
and [IRS section 1341 discussion](https://www.irs.gov/irb/2013-51_IRB).
Social Security lump-sum elections and cross-year
election dependencies, treaty treatment or net-benefit repayments, other
early-distribution exceptions, unresolved retirement basis or special treatments, other
original-year income-sensitive items, changed
joint-return ownership, loss carryovers and farm-sale tax deferral still need
further calculation. Missing history never means zero credit. Report repayments
of other earlier-year income in Additional coverage checks, even when each payment
is small. Employee-paid
program contribution recovery, private funds and employer supplemental benefits
also need their own treatment. See [IRS unemployment guidance](https://www.irs.gov/publications/p525).

Retirement calculations include
qualified charitable distributions coordinated with prior IRA basis and
current contributions whose deduction and exclusion pass the consistency check
(including a shared-income search for both contributing spouses), selected
early-withdrawal exceptions (including qualified public-safety separation, ESOP dividends,
federal phased retirement, governmental 457(b) source allocation and permitted
automatic-enrollment withdrawals), and
owner-specific IRA basis allocation, including current contributions and payment
timing, with on-screen carryforwards. Each filer can keep the default maximum
IRA deduction or designate part or all of the contribution as nondeductible.
Employee-funded section 501(c)(18)(D) trust contributions are entered once in
Adjustments using W-2 code H and annual compensation/deferral records. W-2 wages
remain unchanged. The App checks the separate $7,000 and 25% compensation limits
and the [2026 shared regular-deferral limit](https://www.irs.gov/pub/irs-drop/n-25-67.pdf),
then calculates the separate deduction and includes the contribution in Saver’s
Credit automatically. Do not repeat it in that credit's workplace totals.
Deductible contributions establish IRA workplace coverage and reduce traditional
IRA contribution capacity. They do not reduce IRA or rental-allowance modified
income. Coordination with the same employee’s Roth contributions or a partially
phased-out traditional IRA deduction remains unfinished. Excess contributions
and unresolved catch-up classifications still require correction or further review.
See [IRS contribution reporting guidance](https://www.irs.gov/publications/p525).

Permissible automatic-enrollment withdrawals entered in Income now join the
Saver’s Credit lookback automatically, including earnings. Do not enter them
again in the credit's distribution history or subtract them again from original
2026 deferrals. Other counting distributions still need that history. The existing
[Form 8880 rules](https://www.irs.gov/pub/irs-pdf/f8880.pdf) apply recipient,
joint-return and contribution limits; exemption from early-withdrawal tax does
not itself exempt a distribution from the credit lookback.
Allocated taxable IRA amounts now feed education, medical, unemployment-insurance,
disability, terminal-illness, qualified-reservist and full-levy exceptions without
exempting ordinary income. The same calculation handles the 10% and applicable
25% SIMPLE additional-tax rates, following
[Form 5329](https://www.irs.gov/instructions/i5329).
IRA first-home, birth/adoption, emergency, domestic-abuse and partial-levy
exceptions also allocate basis. Results distinguish gross qualifying withdrawals
from their taxable relief; shared first-home and parent/child limits consume gross
amounts and favor claims saving the most additional tax. The qualified employer-plan
withdrawal worksheet also supplies the taxable proportion for birth/adoption,
emergency, domestic-abuse and levy exceptions, using its same-account balance
and unrecovered investment under [Publication 575](https://www.irs.gov/publications/p575).
Complete qualified employer-plan cash discharges also use the known recovered
basis to allocate these gross-limited exceptions. Other partially taxable pension
sources, overlapping public-safety insurance
exclusions and multiple domestic-abuse claims still require further coordination.
Supported IRA repayment worksheets are described below.
Income limits may make more nondeductible than elected; these contributions
require Form 8606 reporting. Qualified reservist repayments to a traditional IRA
use original distribution, service and dated deposit records. They add
nondeductible basis outside annual contribution limits, without reversing the
original income or creating a deduction, following
[Publication 590-A](https://www.irs.gov/publications/p590a). Results show repayments,
remaining unpaid amounts, current nondeductible basis and the usual basis
carryforward. The calculation checks the two-year deadline and coordinates later
withdrawals, QCDs and supported HSA funding order. Special 2026 traditional IRA birth/adoption, emergency, domestic-abuse and
terminal-illness repayments use their separate
[Form 8606 worksheet](https://www.irs.gov/instructions/i8606). Actual deposits link
to the original income entry. The App checks the three-year deadline and qualified
gross ceiling, adjusts same-year account value, and calculates the owner's taxable
income reduction and future basis adjustment. It pools all applicable qualifying
distributions, even if only one was repaid. Later-year deposits can require an
amended original return; results identify them. Prior-year source distributions,
employer-plan and Roth repayment treatment, disaster repayments and use of earlier
repayment basis adjustments remain outside this worksheet.
Inherited traditional IRA distributions use a separate annual record for each
beneficiary and deceased owner, combining that person's inherited traditional,
SEP and SIMPLE accounts. Allocated nondeductible basis is recovered proportionally
using annual withdrawals and year-end value; results show taxable income and
remaining basis. Personal IRA contributions and other inherited groups stay
separate. Confirmed basis-free distributions are fully taxable, while unknown
basis remains unresolved. Beneficiary payments have no early-distribution tax;
RMD shortfalls remain a separate calculation. Inherited traditional IRA gifts
link to their income entry and use that deceased owner's pretax capacity before
basis recovery. They share the filer's QCD annual limit and deductible-contribution
adjustment with personal IRA gifts; the beneficiary must be at least 70½ on the
payment date. See [IRS Notice 2007-7, Q&A-37](https://www.irs.gov/irb/2007-05_IRB).
Inherited IRA estate-tax deductions use the estate administrator's tax
recomputation, estate-wide IRD value and this beneficiary's unused allocation.
The App limits the deduction to current taxable inherited income, applies it
only when itemizing, and reports remaining estate-value allocation separately
from IRA basis. It does not reduce AGI and remains deductible for AMT when
itemizing. See [IRS Publication 559](https://www.irs.gov/publications/p559).
Shared QCD exclusions without a determined source allocation, Form 8615 with
this deduction, non-IRA and surviving-annuitant estate-tax calculations,
and special repayments still require further support.
Traditional-to-Roth IRA conversions include
partial conversions, basis allocation and regular Roth contribution eligibility.
Ordinary own-account Roth IRA withdrawals now use each owner’s combined history:
regular contributions first, then conversions by year (taxed portion first), then
earnings. Current-year contributions and conversions come from their existing
entries. Age-59½ and permanent-disability treatment distinguish qualified
withdrawals from taxable earnings before the five-tax-year period ends. Recent
conversion principal may incur additional tax without being taxed as income
again; Roth and traditional IRA amounts share each owner’s Form 5329 rounding.
Inherited Roth distributions use a separate annual record for each beneficiary
and deceased owner, combining that person’s inherited Roth accounts. The original
owner’s five-tax-year clock continues; before it matures, allocated regular and
conversion basis comes out before taxable earnings. Beneficiary payments incur
no early-distribution tax. Results show recovered and remaining inherited basis,
gross distributions and withholding. Missing history remains unresolved. Roth SIMPLE IRAs, special
rollovers/repayments and other Roth withdrawal exceptions remain unfinished. See [Roth distribution rules](https://www.irs.gov/publications/p590b)
and [Form 8606](https://www.irs.gov/instructions/i8606).
Required-distribution shortfall tax uses administrator minimums or calculates
own traditional IRA minimums from December 31 balances, outstanding incoming
rollovers and IRS Uniform Lifetime/Joint and Last Survivor tables. The younger-
spouse table requires a qualifying sole-beneficiary spouse more than ten years
younger. Each account uses its own beneficiary facts before totals are combined.
First-year choices distinguish a 2025 payment deferred to April 2026 from a
2026 payment deferred to April 2027. Payments satisfying each deadline remain
separate from late corrections and unrelated account groups. It calculates 25% or the corrected 10% rate, checks the
correction and filing dates against the correction window, and keeps spouses
separate. Planned filing produces an explicitly conditional estimate. Reasonable-
cause waiver requests reduce the reported shortfall pending IRS approval and
retain the explanation; the App does not submit it. Unrelated excess withdrawals
cannot erase a shortfall, and corrective distributions remain income in their
actual payment year. For original nonspouse beneficiaries, inherited IRA minimums use the Single Life
table, the owner's required beginning date,
exact birth-date eligibility, qualifying medical/child status and permitted
ten-year elections. Results distinguish annual payments from the final emptying
deadline, including the child's age-21 transition for post-2019 inheritances.
Established pre-2020 life-expectancy payouts use the 2022 table reset and do not
acquire a retroactive ten-year limit. Once life expectancy is exhausted, the
worksheet requires the actual year-end remaining assets, including earnings.
Sole surviving spouses who retain inherited Single Life treatment use current-age
life expectancy each year, compared with the deceased owner's remaining expectancy
when required. Before-beginning-date inheritances can delay annual payments until
the later of the year after death or the original owner's applicable RMD age.
A missed sole-spouse minimum requires reconciliation of possible deemed ownership;
it is not automatically taxed as a nonspouse shortfall.
Expired five-year elections, other spouse elections and successor beneficiaries,
trust/multiple-beneficiary rules, annuities,
deceased-owner allocation and unpublished younger-spouse ages remain unfinished;
use a verified administrator minimum for those cases. See
[Form 5329](https://www.irs.gov/instructions/i5329).

IRA rollovers preserve basis, distinguish amounts retained from amounts transferred,
and account for eligible transfers completed after year-end.
Eligible postsecondary expenses can reduce the early-IRA withdrawal tax, with
tax-free assistance and room-and-board limits applied separately from ordinary
income tax. IRA health-insurance withdrawals during qualifying unemployment
use benefit-duration and reemployment-date checks, including confirmed eligibility
where self-employed status alone prevented unemployment compensation. The
calculation does not invent benefit income. Eligible unreimbursed medical
expenses can also reduce the withdrawal tax after the AGI floor, even without
itemizing; multiple claims share one allowance. Ordinary first-home IRA
withdrawals share the owner’s remaining lifetime limit and check the 120-day
payment deadline, including payments in the following year. QDRO and qualifying
reservist withdrawals use their account-type and service-order rules. Qualified
birth/adoption withdrawals share a limit for each parent and child and check
the one-year withdrawal window. Emergency-expense withdrawals check account
balance, prior replenishment and the one-withdrawal annual limit. A first, single
domestic-abuse withdrawal uses the 2026 cap and one-year window. Traditional
IRA repayments use the worksheet described above; repeated domestic-abuse
claims and employer-plan or Roth repayment coordination remain unfinished.
Qualified 2026 long-term-care distributions from participating defined-contribution
employer plans can reduce the additional withdrawal tax while leaving pension
income taxable. The App checks the issuer statement, annual certified premiums,
10% vested-benefit ceiling and $2,600 limit under
[IRS Notice 2026-33](https://www.irs.gov/pub/irs-drop/n-26-33.pdf). Repeated payments
share the allowance; spouse coverage requires a joint return. IRAs do not qualify,
and an ordinary withdrawal cannot be retroactively designated when the plan does
not offer this option. The current calculation supports one participating plan
per employee, using a payer-reported taxable amount that already includes the
special pretax-first allocation, fully taxable payments or complete contract
discharges. Calculated partial-basis withdrawals, multiple-plan coordination,
designated Roth and governmental 457 source coordination remain unfinished. This exception has no three-year repayment
option and does not automatically create a premium deduction.
Qualifying employer disability pensions before minimum retirement age flow
through wage reporting, earned-income calculations and the disability credit.
After minimum retirement age, qualifying disability pensions stay on the pension
line while Schedule R uses its separate mandatory-retirement-age rules.
Retired public-safety officers can elect the insurance-premium exclusion against
eligible governmental pensions, including disability pensions. The App applies
one $3,000 limit per officer across plans after basis recovery and carries only
remaining eligible premiums into medical itemization. Qualified long-term-care
premiums share the 2026 age limit for each insured person. The election and its
allocation order are explicit; source payments and premiums are entered once.
Savings-withdrawal penalties, qualifying alimony and third-year alimony recapture
also flow through income, adjustments and applicable IRA limits. Jury pay and
amounts surrendered to an employer for continued salary appear separately as
income and an adjustment. Dependent-care
credits include qualifying 2025 expenses first paid in 2026, using prior-return
limits and rates. Ordinary employer dependent-care benefits for one filer
coordinate exclusions, taxable wages and care-credit expenses; more complex
benefit arrangements remain unfinished. Payroll-based monthly student and
self-care facts provide deemed earnings for care limits without adding taxable
income. Monthly business, foreign-pay and special benefit allocations remain
unresolved. Casual gambling records feed winnings, withholding and the 2026
itemized loss limitation. For young filers, losses actually deducted also
reduce ordinary unearned income in Form 8615 after applicable limits; they do
not directly reduce qualified dividends or net capital gains. An ordinary professional gambling activity uses the
shared business calculation, including its wagering-expense limit, SE tax and
QBI. Mixed activities and special gambling deductions remain unfinished.
Adoption assistance uses per-child employer benefits and prior exclusion limits,
separate from unreimbursed credit expenses. Where limits are shared with another
return, enter the agreed credit and exclusion shares independently; prior claims
reduce those shares before the income phaseout. On-screen results show each limit.
The App applies the 2026 exclusion
cap and income phaseout, distinguishes exclusion income from credit income,
and displays Form 1040 line 1f, including eligible negative special-needs
adjustments. Current foreign benefits stay taxable until finalization. IRA,
Social Security and rental-loss modified-income calculations add the exclusion
back where required. Earlier taxable foreign benefits enter the finalization-year
exclusion worksheet without being taxed twice. Cases producing a prior-year wage
reduction, multiple employees for one child and unresolved coordinated worksheets
remain unfinished. The calculation uses the published
[Form 8839 instructions](https://www.irs.gov/instructions/i8839) with
[2026 inflation parameters](https://www.irs.gov/irb/2025-45_IRB);
final 2026 form reconciliation remains pending. Separate filers can claim current
adoption benefits when they lived apart for the last six months, paid over half
the home costs and the child lived there for more than half the year. An eligible,
documented prior credit share can be used independently of these current-household
tests; an unallocated joint-return balance cannot be duplicated on separate returns.
Beneficiary-funded ABLE contributions share the Saver’s Credit limit and
distribution lookback with retirement contributions. ABLE distributions use gross withdrawals, reported earnings and qualified
disability expenses to calculate taxable earnings and the separate 10% additional
tax. Qualified transfers and post-death payments have distinct choices.
Timely returned excess or additional-account contributions tax the contributor's earnings for the
contribution year without the 10% distribution tax. Earlier-year corrections flag
that year's return for review. A separate contribution worksheet applies the
2026 $20,000 base limit, eligible working-beneficiary allowance and 6% excess tax,
capped by year-end account value. An optional first-60-days expense election
carries a reminder not to reuse those expenses next year. Mixed withdrawals and
qualified rollovers use the source payment's earnings allocation before calculating
the expense exclusion. Beneficiary changes distinguish eligible siblings from
nonqualified successors; a nonqualified change adds the account's change-date
value and earnings to the former beneficiary's annual distribution calculation.
Actual and deemed withdrawals share one expense exclusion. Uncorrected second
accounts, unresolved late corrections and
investment-tax coordination remain unfinished; affected estimates show an explicit
issue.
Complex exceptions remain explicit; this is not every federal tax scenario.

Foreign housing deductions, adverse-condition second households and treaty treatment, complex Form 1116 allocations, rental
personal use/disposals/special loss histories, special Marketplace allocations,
QCD deduction/exclusion combinations that fail the consistency check, special IRA distributions, employer-plan
and inherited-account conversions,
and IRA-to-plan rollovers requiring transaction-date valuation,
and additional AMT adjustments still require
further worksheets. Unsupported combinations produce review issues instead of
completed tax or refund totals.

<h2 style="font-size:2rem;line-height:1.2;">One App, a complete interaction</h2>

- **A guided interview.** The server supplies the next editable request, including
  the values and context needed to continue.
- **Saved progress.** Accepted answers persist, so reopening the task resumes
  the interview. Client form caching can restore committed field edits.
- **Calculated results.** The App applies its implemented tax rules and returns
  on-screen return lines, supporting calculations and issues for review.

Libertas builds the forms from the App's declarations. The App owns the tax
logic and interview order; it needs no separately authored mobile forms.

<h2 style="font-size:2rem;line-height:1.2;">What the line count includes</h2>

The current example has **35,024 lines across 71 Rust application source
files**, including IRS rate tables, comments, annotations, and blank lines. Its separate test
file adds 41,654 lines. Generated documents and the SDK are outside this count.

Employee-tip records now support additional wages outside W-2 box 1. Enter
actual monthly cash/charged/shared tips and the portion already reported, or
use allocated tips from box 8 when adequate actual records are unavailable.
Actual records replace the allocated estimate. Noncash tips are separate.
The App applies the $20 monthly cash-tip threshold per employer, calculates
each spouse's Form 4137 payroll tax, reserves the Social Security wage base
before self-employment income, and includes covered tips in Additional Medicare
tax without inventing withholding. Government Medicare-only coverage is distinct.
Tips feed earned-income and compensation limits; eligible reported/Form 4137
cash tips may also enter the existing qualified-tip deduction. On-screen
Form 1040 results keep box 1 wages on line 1a and additional tips on line 1c.
W-2 box 12 codes A/B (reported tips) and M/N (former-employee group-term life
insurance) now have separate uncollected-tax inputs. They add tax without
repeating income. Ordinary Medicare amounts reconcile Additional Medicare
withholding; Social Security amounts enter the multiple-employer excess-tax
credit. These records can coexist with additional unreported tips. Unresolved
reporting-year differences, railroad treatment, employer corrections and other
special payroll cases still need separate records. The calculation uses
[Form 4137](https://www.irs.gov/pub/irs-pdf/f4137.pdf) and the
[2026 payroll parameters](https://www.irs.gov/publications/p15); final 2026
Form 4137 reconciliation remains pending.

Employee compensation omitted from W-2 now has a separate Form 8919 source.
Enter the applicable IRS determination, correspondence, pending SS-8, or same-firm
W-2/1099 reason and the omitted pay. It remains employee income, not Schedule C
receipts. Calculations combine firms separately for each spouse, apply the remaining
Social Security base after reported wages and unreported tips, and include Medicare
and Additional Medicare tax. On-screen results show these wages on line 1g.
Employee compensation also enters earned-income and applicable contribution limits.
Do not duplicate the payment in another income document. The App does not file
SS-8 or decide employment status; pending IRS determinations remain explicit.
See [Form 8919](https://www.irs.gov/pub/irs-pdf/f8919.pdf). Its currently published
2025 instructions supply the calculation sequence, using the verified 2026 payroll
parameters above; final 2026 form reconciliation remains pending.

Household employers can now enter payroll and state unemployment records for
Schedule H. The App calculates employer and employee payroll taxes, applies
family and student exceptions, and calculates FUTA from current/prior-year
quarterly wages. State experience rates, actual payments and payment dates drive
the credit; late payments and wages in multiple states have separate treatment.
Employee income tax withheld increases Schedule H liability, not your personal
withholding credit. Results show the supporting taxes and include them in the
on-screen return. See [2026 Publication 926](https://www.irs.gov/publications/p926)
and [Schedule H instructions](https://www.irs.gov/instructions/i1040sh).

Final 2026 FUTA credit-reduction rates are required wherever relevant; unknown
rates remain unresolved rather than becoming zero. DOL currently publishes
[potential 2026 rates](https://oui.doleta.gov/unemploy/futa_credit.asp), with final
determinations after November 10. Final 2026 Schedule H reconciliation also
remains pending. State disability-plan notices now support ordinary payroll-taxable
payments and employee taxes already deposited by the state. The App combines
those payments with regular wages for thresholds and caps, then subtracts the
actual state payroll deposits from Schedule H liability. State-withheld income
tax does not become your personal payment credit. Use the notice's taxable
portion, not the gross benefit; see the [Schedule H state-disability instructions](https://www.irs.gov/instructions/i1040sh).
Alternatively, gross state sick-pay amounts can be calculated from plan records.
The App uses employer versus employee after-tax funding, actual last-work dates,
Social Security disability entitlement and death/disability-retirement facts.
It applies the six-calendar-month limit, restarting after any later work.
Group-policy funding uses the last policy-year net costs known at the start of
2026, or the permitted first-year estimate/actual-cost determination when no
policy-year costs were known. Individual-policy funding uses premiums for the
policy year containing each payment, with separate records when that year changes.
Uninsured funds use actual prior calendar-year contributions; new funds use
actual current-year contribution determinations, including periodic determinations
applied to the succeeding period. Missing or overlapping periods are rejected. Prior-year Social Security disability entitlement can exclude FICA
while leaving FUTA payable. Actual state tax deposits remain source facts.
See [2026 Publication 15-A](https://www.irs.gov/publications/p15a).
See the [insured-plan allocation rules](https://www.govinfo.gov/content/pkg/CFR-2025-title26-vol2/pdf/CFR-2025-title26-vol2-sec1-105-1.pdf).
Unresolved liability, private-plan transfers,
corrections and special territory/treaty cases still need further treatment.
Do not enter taxes already reported and paid for you by an authorized government
agency or agent: those taxes are not reported again on Schedule H. Hiring a
payroll service alone does not remove your responsibility. See the
[Schedule H agency instructions](https://www.irs.gov/instructions/i1040sh).
Payroll operations and employee W-2 filing are not part of this prototype.

Cash-method farms enter raised-product sales, purchased-product resale
proceeds and basis, custom-hire receipts, ordinary taxable program payments and
other operating income. Shared business calculations apply expenses, supported
equipment deductions, SE tax, owner health/SEP deductions and QBI; the result
shows farm gross income, costs and net profit separately. Accrual farms instead
enter recognized sales and other income, opening/closing tax inventories and
product purchases. Inventory changes feed gross income once; incurred expenses
follow accrual timing, including economic-performance and related-party rules.
Current farm losses
can offset other income. When a farm NOL arises, an explicit, confirmed timely
carryback waiver now preserves the regular/AMT carryforwards and displays the
required election-statement reminder. The App does not choose or file that
election. The two-year carryback still needs prior-return recomputation; see
the [Form 172 election instructions](https://www.irs.gov/instructions/i172).
Crop-insurance records now distinguish physical-loss proceeds from price-only
or index-based payments. An eligible cash-method election defers all qualifying
current-year payments for that business to 2027. Earlier damage-year receipts
and prior elected amounts remain taxable now; actual 2026 withholding remains
a current payment credit. Accrual farms include recognized insurance income
without this cash-method deferral. The result shows included and deferred
amounts and the separate election-statement requirement. See
[IRS crop-insurance timing](https://www.irs.gov/publications/p225).
CCC loan records distinguish current advances from earlier loans already
included in income. Cash farms support loan treatment or the section 77 income
election. Accrual farms support loan treatment, with all pledged or redeemed
crop inventory included in the shared opening/closing inventory calculation
and recognized CCC sales entered only in the lot records. Redemption adjusts taxable market gain or crop basis
according to that history; forfeitures and later sales enter income once.
Forfeiture records also track the settlement year and any surplus the CCC pays
in 2026, so a later payment is included without repeating earlier principal.
A shortfall released by the CCC does not create a deduction. For a loan already
included under section 77, a deficiency the farmer is legally required to pay
and actually pays during 2026 enters farm expenses once, including payments
after an earlier disposition. The review shows these components, retained basis
and current withholding. One
section 77 method applies across the taxpayer's farms and continues into later
years; required election statements remain the taxpayer's responsibility.
A partial loan release uses separate records for each source-identified released
portion and the remaining collateral, with its own attributable original
principal. Include every portion of a current advance; never repeat the full
loan amount for each portion or invent a basis allocation. The display separates
full reported forfeitures from taxable forfeitures. Accrual section 77 inventory
and basis coordination, unidentified partial payments, deficiencies under loan
treatment and additional basis adjustments still require unfinished worksheets. See the
[IRS CCC loan rules](https://www.irs.gov/publications/p225).
Cash farms can enter unused production-supply prepayments and retained
undeducted costs separately from ordinary expenses. The App calculates the
expense ceiling, evaluates supported farm-related exceptions and releases
identified earlier costs when consumed. Current costs enter farm expenses once;
the review shows the deduction and remaining balance. Purchase, business-purpose
and income-distortion facts remain necessary even when an exception removes
the ceiling. Ordinary depreciation, the allowed simplified home-office deduction
and supported final-return conservation adjustments enter the expense base.
The App resolves the supply and home-office limits together, without deducting
either cost twice. Poultry bought solely for resale uses identified purchase
lots and the consistently adopted cost method. Under eligible payment-year
treatment, current unsold costs share the ceiling. Under sale-year treatment,
unsold costs stay in basis even if a prepaid exception applies. Costs not
previously deducted are released on sale or other disposition. The review shows
those costs separately; enter sale receipts without deducting the same basis
again. Retained production-poultry costs paid in 2025 and deferred solely by
that year's prepaid ceiling are released in 2026 without waiting for sale.
The App subtracts previously deducted costs, adds the release to other expenses
and applies the current ceiling once. Current production-poultry purchases and
unresolved ceiling coordination with section 179 or differing AMT depreciation
still need further worksheets. Multiple cash farms owned by the same taxpayer
share the expense ceiling when every eligible prepayment fits or only one farm
has eligible new prepayments. Deductions and remaining costs stay with their
source farm. A binding ceiling with competing farms, different owners or mixed
accounting methods still needs additional coordination. Document-only previews
defer these calculations to the complete return.
Home-office use combined with a farm-land disposition also requires the
unimplemented gain/loss attribution to the office income limit. Qualifying ceiling exceptions can
use the existing shared deduction calculations. See the
[IRS prepaid-supply rules](https://www.irs.gov/publications/p225).

CRP annual and incentive payments now retain their taxable farm income while
separately excluding qualifying payments from self-employment earnings. Benefit
status is recorded for each payment date. The exclusion also flows through
applicable earned-income and owner deduction limits; current withholding remains
credited. Cost-sharing exclusions and nonbusiness landowner treatment still need
separate worksheets. See [Schedule SE line 1b](https://www.irs.gov/instructions/i1040sse)
and the [IRA compensation exclusions](https://www.irs.gov/publications/p590a).
Each filer can now elect the farm, nonfarm, or combined optional self-employment
method. The calculation checks gross income, profits and nonfarm prior-use records,
shares the $7,560 cap, and retains actual taxable business income. EIC and the
additional child tax credit use their separate earned-income rules. Health-insurance deductions now use optional earnings attributable to the insured
business, including loss businesses and shared-cap allocations, under
[Form 7206](https://www.irs.gov/pub/irs-pdf/f7206.pdf). Owner-only SEP, IRA and Archer MSA contribution limits retain actual business
income and incorporate the resulting half-SE deduction; optional earnings do not
create extra contribution room. See the [SEP worksheet](https://www.irs.gov/publications/p560),
[IRA compensation rules](https://www.irs.gov/publications/p590a) and
[Archer MSA income limit](https://www.irs.gov/publications/p969). These amounts
use the [2026 draft Schedule SE](https://www.irs.gov/pub/irs-dft/f1040sse--dft.pdf);
final IRS reconciliation remains required.
Ordinary directly owned farms can now make an explicit income-averaging election
using retained 2023–2025 return facts. The App calculates the eligible amount,
adjusts negative base-year income, uses the latest applicable prior Schedule J,
and shows the chosen election beside tax without averaging. An election that
raises tax produces a review note; the App does not silently replace your choice.
AGI, self-employment tax, QBI and carryovers stay unchanged. AMT uses the regular
tax calculated without averaging, as required by
[Form 6251 line 10](https://www.irs.gov/instructions/i6251).
Retained Form 4952 investment-interest elections also adjust base-year
preferential income, including an explicit capital-gain/dividend allocation.
The App reads the historical election rather than making a new one.
Current-year foreign employee-income exclusions can accompany ordinary farm
averaging. The reduced-income calculation retains exclusion stacking and
recomputes any excess preferential income; AMT still uses unaveraged income.
Foreign collectible and unrecaptured section 1250 gains now use separately
recomputed regular/AMT excess amounts, including when farm averaging reduces
current taxable income. The eligible small foreign-tax-credit election from
payee statements also works with averaging, limited by the elected regular tax;
full Form 1116 allocation remains unfinished.
Retained base-year Form 2555 exclusions and housing deductions now participate
in averaging, including related disallowed deductions and capital-gain-excess
adjustments. They do not reduce current-year AGI. A sole farmer can use actual base-year
filing statuses even when they differ from the current return, including a
previous joint return or a current joint return with earlier individual returns.
Retained records must follow that farmer; the App does not substitute the other
spouse’s history.
Farm averaging now uses the farm components of the existing QBI calculation
when those components reconcile to the final deduction. Nonfarm business and
REIT deductions do not reduce eligible farm income; qualified business losses
are netted once. Mixed-business cases where the overall income cap or minimum
deduction changes attribution still require further calculation.
Personal itemized deductions can accompany averaging when no state/local income-tax
allocation is needed. Automatic deduction selection compares total tax, including
AMT, rather than choosing the largest deduction. A review note identifies when a
route is unavailable; that comparison cannot establish the lowest tax across
all possible routes. Explicit deduction choices are preserved.
Farm capital gains, fishing and other farm sources,
itemized state/local income-tax allocation, prior NOL allocation, Archer MSA coordination, changes
requiring separate histories for two farming spouses, full Form 1116 and young-filer
coordination still require further averaging worksheets. Current-year results
remain estimates pending final 2026 reconciliation. See the
[Schedule J instructions](https://www.irs.gov/instructions/i1040sj).
Soil-and-water conservation costs now have an explicit capitalization or
section 175 deduction treatment. The App combines supported gross product income
from the same filer's directly owned farms before applying the 25% limit. It
supports all costs when the shared limit does not bind, or costs belonging to
one farm when it does. Allowed expenses enter once, with separate carryforward
and land-basis outputs.
The deduction requires confirmation of the qualifying plan and valid method.
District depreciable-property assessments now apply the total-share cap,
combine same-year installments, and spread qualifying excess over the following
nine years. Current deduction, deferred amounts, income-limited carryforward and
land-basis additions remain separate. Enter the original district assessment and
payment records, including earlier years; do not enter calculated deductions.
A deceased farmer's remaining prior-year assessment spread now becomes available
on that owner's final return, still subject to the farm-income limit. Unused
income-limited amounts expire rather than transfer to the estate. A living
farmer's balance is not accelerated because their spouse died.
A binding limit across costs from several farms, different farm owners,
nonproduction receipts/subsidies, other farming sources, revised assessments,
current-parcel deduction/disposal coordination and a new assessment spread beginning in the death year
remain unfinished.
See [Publication 225, conservation expenses](https://www.irs.gov/publications/p225).
A per-sale farmland tax-installment election now calculates supported ordinary
regular-tax cases from qualifying use, buyer and covenant facts. All selected
sales share one calculation, retaining actual AGI and deductions while refiguring
gain character and personal-credit limits. The screen separates total tax, the
first installment, the remaining deferred liability and current tax payable.
Withholding and actual payments remain separate from the deferral offset.
For example, a supported $40,000 capital-gain sale produces $6,000 attributable
tax: $1,500 is due initially and $4,500 remains deferred. The return's other tax
is still due normally. Actual credit carryovers and refundable credits are not
replaced by the hypothetical reduced-tax worksheet.
Eligible small foreign-tax-credit elections from payee statements now work with
this calculation. The credit is re-limited before personal credits; its actual
return amount remains unchanged and no election-year carryover is created.
Premium-credit repayment also participates in both regular-liability comparisons,
using unchanged household income and coverage facts. Foreign employee-income
exclusions retain tax-rate stacking and recompute preferential-income caps for
the reduced-tax comparison.
Supported Schedule J averaging can also accompany the installment election:
the comparison preserves the chosen amount up to its reduced taxable-income
ceiling. If that ceiling binds, the App recomputes the three base-year allocations
from their retained sources, including supported foreign and preferential-income
worksheets. The actual election, source records and annual tax remain unchanged.
AMT stays in the actual return. Personal credits allowed against it are preserved
in the actual comparison; the recalculated credit total is capped at reduced
regular tax as Form 1062 line 12z requires. The comparison does not change actual
credit carryforwards or refundable credits. Section 53 minimum-tax credit is
excluded from both Form 1062 comparisons: it remains calculated on the actual
return, with its actual carryforward. No hypothetical minimum-tax-credit worksheet
is required for this election. The deferred amount remains a separate payment
offset even when it exceeds annual tax after credits. The resulting negative
current tax position contributes to the estimated overpayment; it does not reduce
annual tax below zero or cancel later installments. Full Form 1116, parent-rate tax and final-return
coordination also remain unfinished for this election
and prevent final totals. Leaving the election absent keeps ordinary payment
treatment. A direct farm sale after its linked owner's death is rejected from
that owner's final return; a surviving spouse's own sales are checked against
that spouse's dates. Pre-death receipts still require confirmed final-return
allocation under [Publication 559](https://www.irs.gov/publications/p559). The App does not file the election or covenant or make payments.
See [Form 1062](https://www.irs.gov/instructions/i1062). Provisional line mapping
uses the [2026 draft Form 1040](https://www.irs.gov/pub/irs-dft/f1040--dft.pdf)
and [Schedule 3](https://www.irs.gov/pub/irs-dft/f1040s3--dft.pdf); final-form
reconciliation remains required.

Direct sales of purchased or qualifying inherited farm land now calculate gain from closing proceeds,
selling expenses and retained basis. Inherited land with estate fair-market-value
basis receives long-term treatment without changing the actual acquisition date;
only the heir's own later conservation deductions carry into recapture. Prior allowed conservation deductions feed
section 1252 recapture. Choose confirmed identical property records or enter the separate AMT basis and
allowed conservation deductions. Complete regular/AMT section 1231 histories
determine ordinary and capital character independently. A purchased parcel held
one year or less is ordinary income and does not require unrelated section 1231
history when no regular or AMT section 1231 amount exists. Supported active-farm sales feed income, QBI,
NIIT, section 179 limits and final loss histories without becoming Schedule F
receipts or self-employment income. Enter each sale once. Installment-sale proceeds,
the unresolved tax-election combinations above, former-passive use, current-parcel conservation deductions, unresolved
multi-business lookback/excess-loss allocations and Schedule J combinations with
ordinary land-sale QBI still need additional coordination. Capital-only land sales
can accompany Schedule J: the land gain stays out of elected farm income and is
taxed in the current year. For one farm with a loss in only one net character,
excess-business-loss deferral now preserves the correct property history and
separate QBI/NOL balances; mixed unresolved loss allocations remain blocked.
See [Form 4797 instructions](https://www.irs.gov/instructions/i4797).

Unresolved carrybacks, accounting-method changes, cooperatives, other special
subsidies and remaining special expense elections also remain unfinished and cannot
produce a completed return. See [Schedule F instructions](https://www.irs.gov/instructions/i1040sf)
and [Publication 225](https://www.irs.gov/publications/p225); final 2026 form
reconciliation remains pending.

The S-corporation K-1 interview now calculates ordinary U.S. business income
and losses through regular and AMT basis, at-risk and passive limits, QBI and
applicable return-level calculations. The supported source confirmation explicitly
excludes farming; farm carrybacks and farm/nonfarm loss allocation remain unfinished. Box 13 code O backup withholding is a
separate payment and increases federal withholding without reducing business
income or QBI. The review identifies its filing-attachment requirement.
Boxes 4, 5a and 5b supply taxable portfolio interest, total dividends and the
qualified-dividend subset. They increase shareholder basis and use the shared
interest/dividend return calculations, including preferential rates and
investment tax. They never become passive operating income or QBI. The current
source confirmation requires these amounts to belong to the same at-risk
activity; that activity's loss limit includes them, while passive losses still
cannot offset them. Separate at-risk activities and special foreign or
self-charged-interest cases need additional allocation worksheets. See the
[K-1 instructions](https://www.irs.gov/instructions/i1120ssk) and
[Form 6198 instructions](https://www.irs.gov/instructions/i6198).
Boxes 7, 8a, 8b and identified portfolio box 8c sources have capital records, with separate
short-term, other long-term and collectible basis/at-risk suspended losses and
independent AMT amounts. The signed box 8b collectible component is already
included in box 8a; the App separates it for the 28% rate worksheet without
adding it to income again. Prior collectible losses are likewise components of
the retained long-term totals. On-screen results show both the separate
components and combined long-term suspended balances for the next year.
Ordinary and capital losses share the applicable limits proportionally. Only
allowed capital amounts reach the shared Schedule D calculation; its return
carryforward remains separate from losses still suspended at the K-1 level.
Mixed gains and losses retain their short-/long-term character. Positive K-1
capital income increases basis before current and carried loss items share the
basis limit; those losses are not first offset against another capital category.
The allowed amounts then enter the return's capital netting, including the
collectible rate calculation after losses. Portfolio unrecaptured section 1250
amounts from partnership-interest sales and estate/trust/fund/REIT sources are
separate components of box 8a, taken from the attached box 8c statement. They
enter lines 10 and 11 of the section 1250 worksheet without adding income again.
Regular tax and AMT compute their 25% components independently, including loss
offsets. A bare box 8c amount does not identify those portfolio sources.
Business-asset section 1250/section 1231 amounts have a separate source record
for box 9, the business portion of box 8c, and prior basis, at-risk and passive
suspensions. Its on-screen basis and at-risk results preserve those balances
separately, with independent AMT records. For a current passive activity, its
ordinary and business-property amounts now share the passive pool while retaining
separate character and suspended-loss balances. Current nonpassive amounts remain
outside that pool. Former activities release prior losses against their own net
income first. Partial releases across both historical characters offset income
against matching-form history before allocating the remaining suspension.
For opposite-signed current amounts, the current loss first reduces the other
character's income; only net current income releases old losses. Current
nonpassive losses are not suspended as passive losses. Remaining history can use
other passive income. With no current net income, both prior-loss characters
enter the shared pool. These on-screen allocations are intermediate; unresolved
pool inputs can change them. Fully resolved property-gain cases now produce final
on-screen estimates after QBI, investment-tax and return checks. Unresolved
calculations identify their specific missing treatment. Character allocation follows
[Form 8582 Part IX](https://www.irs.gov/instructions/i8582).
The coverage review now accepts one return-wide section 1231 lookback record,
with independent regular-tax and AMT loss years. For the same filer or unchanged
joint-filing couple, the worksheet nets the entered K-1 property amounts and
shows provisional ordinary/long-term character, the business-asset section 1250
component, and each older loss balance used, expired or still eligible. Missing
histories are unresolved; confirmed empty lists establish no prior eligible loss.
Completed returns show each next-year lookback balance, including current net
losses after basis, at-risk and passive limits. Suspended losses stay separate.
A current deductible loss can exceed the NOL it produces; the lookback balance
is not another deduction. For ordinary nonfarming nonpassive businesses with only
property losses, the excess-business-loss limit now separates current QBI,
deferred qualified loss and regular/AMT NOL carryforwards. Only the property loss
allowed after that limit enters the current lookback history. Full, partial or
zero QBI qualification uses the resolved source proportion; earlier basis and
at-risk limits still apply first. Released historical property losses retain their
original qualified proportions, including pre-2018 exclusions and prior
service-business eligibility. Unreleased passive history stays suspended. Multiple
property-loss businesses, including both spouses on a joint return, use the
combined limit and retain their resolved qualified proportions. Regular and AMT
can have different source totals or an explicitly zero source. A sole business
with loss in one character and gain in the other also has a determinate
allocation. An operating loss leaves qualified property lookback recapture intact;
a property loss leaves qualified operating profit intact. Only the qualified
portion of the deferred loss is removed from current QBI. Basis limits and current
service-business eligibility determine that portion first. Operating profit can
absorb property loss for the excess-loss limit, but is not section 1231 gain and
does not reduce property lookback history. Historical losses in the loss
character retain their original qualified proportions; still-passive balances
remain suspended. The gain character must have no suspended loss balance.
Losses in both characters and broader gain/loss allocations remain unfinished. See the
[excess-loss ordering rules](https://www.irs.gov/instructions/i461) and
[qualified-loss deferral rules](https://www.irs.gov/irb/2020-29_IRB).
These worksheets follow
[Form 4797 lines 7–9](https://www.irs.gov/instructions/i4797) and remain separate
from final return totals. Filing-status changes, other property dispositions
and unreconciled prior returns require further source work.
The rental-allowance income worksheet projects nonpassive property income
through regular-tax lookback and capital-loss rules, excluding passive property
items. Both regular and AMT rental allowances use that regular-tax projection;
raw property gains are not automatically added at face value.
Ordinary QBI source amounts are also calculated when the same activity contains
property items. Shared basis and at-risk limits remain intact, while the ordinary
passive suspension and qualified-loss history stay separate from property amounts.
The screen distinguishes current ordinary QBI before service-business limits from
released historical qualified losses. Complete supported loss records now produce on-screen return totals.
A separate property-QBI record now traces potential qualified losses through
basis, at-risk and passive limits, preserving each loss year's original qualified
percentage and keeping property balances separate from ordinary losses. Its
character worksheet distinguishes qualified loss from the current source year
from released historical qualified losses, even when a later limitation combines
them in a new suspension year. Current
service-business eligibility must be established before fixing new qualified
suspensions; historical eligibility is retained. The record also accepts potential
qualified property gains. These source and loss-tracking results feed final QBI for supported combinations;
unresolved source allocations remain explicit. The return-wide character worksheet now
excludes every constituent property's gain and loss when the net is capital with
no lookback recapture. For a net loss or exactly zero net, it preserves each
business's potential qualified gain and allowed qualified loss separately.
For one business, lookback recapture enters its combined QBI worksheet when the
qualified portion is determined by the source facts: fully qualified gains,
nonqualified gains, or full recapture of a mixed gain. Full recapture across
multiple businesses is also supported when every resolved property result is
nonnegative: each business retains its own ordinary gain, including its
qualified portion and investment-tax classification. Regular and AMT
investment-interest limits use their independent source amounts. Partial
recapture also resolves when the entered section 1250/general gain classes
determine each business's amount under
[Notice 97-59 ordering](https://www.irs.gov/pub/irs-irbs/irb97-45.pdf): the
section 1250 class is recaptured first. This uses reconciled, nonnegative
source results without intervening loss offsets. An optional supporting-statement
amount identifies the qualified part of the section 1250 component; the App
uses that source fact to calculate mixed qualified/nonqualified recapture where
the class amounts determine it. Zero confirms no qualified amount in that
component; an unanswered value does not mean zero. Missing facts, ambiguity
within a class, and unresolved loss offsets still require further source allocation. The equality case follows
[section 1231(a)(2)](https://www.govinfo.gov/content/pkg/USCODE-2024-title26/html/USCODE-2024-title26-subtitleA-chap1-subchapP-partIV-sec1231.htm).
The combined business worksheet counts allocated wages and qualified-property
basis once for ordinary and eligible property items. Current gains and losses
receive current-year service-business treatment; historical qualified losses
remain separate. Resolved property ordinary and capital amounts now enter the
income calculation before combined QBI eligibility, including service-business
phaseouts. Regular and AMT capital results retain independent activity histories.
Verified gain and loss cases reconcile ordinary income, capital income, QBI and
Form 1040 totals, including independent regular/AMT histories and passive/nonpassive NIIT.
EIC investment income excludes section 1231 capital from its capital line and
includes passive property income separately. Prior-return capital records accept
the retained Form 8960 excluded-loss difference; the App applies it to current
NIIT and reports any difference still deferred with capital losses. Supply a
confirmed amount, including zero, unless no regular loss remains or the remaining
losses are confirmed entirely investment-property losses. This does not establish
business/nonbusiness character for NOLs. For a nonmaterially participating owner
of a trading business, eligible ordinary and property amounts now feed Form 4952
with separate regular/AMT character. Capital gains require the existing election;
materially participating owners do not use that business income for this limit.
Earlier ordinary/property passive losses that remain fully suspended do not
block this calculation. Released former-passive losses still need their
investment-expense attribution.
Mixed-source excess business losses, unresolved NOL source allocation, ambiguous recapture
and former-passive trading-property interest attribution still require additional
worksheets and withhold final totals where applicable.
Offsetting qualified and nonqualified components within a single net property item
still require underlying transaction allocations. See the
[Form 8995-A loss-tracking instructions](https://www.irs.gov/instructions/i8995a)
and [section 1231 character rules for QBI](https://www.irs.gov/irb/2019-09_IRB).
Qualified-small-business-stock adjustments and other special capital categories
still need separate worksheets. See the
[holding-period categories](https://www.govinfo.gov/content/pkg/CFR-2025-title26-vol13/pdf/CFR-2025-title26-vol13-sec1-1366-1.pdf),
[basis loss allocation](https://www.govinfo.gov/content/pkg/CFR-2025-title26-vol13/pdf/CFR-2025-title26-vol13-sec1-1366-2.pdf)
and [Form 7203 instructions](https://www.irs.gov/instructions/i7203).
The [K-1 instructions](https://www.irs.gov/instructions/i1120ssk) direct box 8b
to the 28% worksheet after applicable loss limitations.
Box 16 tax-exempt interest and other tax-exempt income are separate inputs.
Both increase basis and the supported activity's at-risk amount; only interest
appears on Form 1040 line 2a and enters tax-exempt-interest calculations such as
Social Security combined income. The specified private-activity-bond portion
is included once in the shared AMT calculation, not added again as income.
**Fully specified supported cases produce
on-screen Form 1040 totals.** Missing records identify the affected document or
worksheet and withhold totals. Other K-1 income, deductions, credits, special
financing, dispositions and ownership termination still require additional
worksheets; the ordinary-business completeness answers must exclude those items.
A deceased shareholder's K-1 remains unsupported pending final-year allocation
and suspended-loss treatment. Shareholder-loan results show ending principal,
adjusted basis, nontaxable repayment and next-year loan classification. An open
account exceeding $25,000 must remain separately tracked in later years, even
if its balance falls. For ordinary gains on open-account repayments, the
at-risk worksheet reduces investment by adjusted debt basis recovered, without
counting the gain as operating income. Ordinary gains from personal-investment
open-account and former-open-account loans now enter Schedule 1 income, the
investment-interest limit, investment tax and independent AMT calculations.
They do not become corporate passive income, earned income or QBI. The source
confirmation excludes purchased discount debt, lending businesses and imputed
interest. Written-note gains use the existing short-/long-term classification
and shared capital-gain, investment-interest and independent AMT calculations.
The optional holding-period input accepts either one established classification
for all taxable repayments or dated principal payments on an existing note with
no new advances. Dated payments can span short- and long-term holding periods;
the App allocates the gain while keeping one debt-basis record. Payments on the
one-year anniversary are short-term; later payments are long-term. Missing
facts or dated payments combined with new advances remain unresolved. A note
must not be artificially split to allocate gain. No holding-period answer is
needed if both regular and AMT repayment gains are zero. See the
[Form 7203 instructions](https://www.irs.gov/instructions/i7203),
[Form 4952](https://www.irs.gov/pub/irs-pdf/f4952.pdf) and
[Form 8960 instructions](https://www.irs.gov/instructions/i8960).
The optional at-risk worksheet uses first-year cash investments or retained
detailed-method history, separates protected funding and repayments of excluded
investment, and reports additional suspended loss and recapture. Noncapital
nondeductible expenses share the at-risk limit proportionally with ordinary
losses once basis permits all current amounts. Their separate regular and AMT
carryforwards never become tax deductions or QBI losses. Simultaneous basis
limitations and basis-only nondeductible carryforwards still need coordinated
allocation. Recapture
still prevents return totals, including recapture found only in AMT records; its
passive/QBI treatment and next-year deduction need further integration. Missing history
is unresolved; it is never treated as fully at risk.
Optional participation facts also show whether an ordinary-business material
participation test is established. Missing history, cross-business aggregation
and special participation cases remain explicit. Confirmed continuing passive
K-1 activities can share a regular-tax passive-loss worksheet with rentals,
after basis and at-risk limits. Separate prior passive-loss records are required.
An unresolved relevant activity prevents presenting the combined worksheet as
complete. For an activity that is now nonpassive, older passive losses first
offset its own current income; remaining older losses join the passive pool.
Current nonpassive income or loss stays outside that pool and affects the
rental-loss income limit. Optional retained QBI loss-year records identify the
qualified portion released through basis, at-risk and ordinary passive limits,
with separate loss-year balances for each limit. On-screen carryforward records
show original loss and original QBI amounts alongside remaining balances, so
the fixed qualified percentage can be retained for later years. Missing current QBI statements
and loss-character histories are identified by document, including when losses
are fully suspended. Confirmed empty history lists differ from missing records.
Those loss-character amounts enter the QBI deduction calculation after applicable
excess-business-loss limits. A current section 199A statement can
also supply ordinary QBI income, allocated wages, property basis and
service-business status. Current business amounts and released historical losses enter the QBI engine
separately. Current service-business losses use this year's income limit before
new QBI carryforwards are recorded; historical eligibility is preserved.
Investment-tax classification separates ordinary nonpassive operating amounts
from financial-trading amounts and preserves the treatment of former-passive
losses. Trading for owners’ accounts is nonpassive even without material
participation, so its current profit cannot release unrelated passive losses.
Actual participation remains a separate fact for the minimum QBI deduction.
Ordinary trading amounts also enter the investment-interest income limit when
the shareholder does not materially participate, using independent regular and
AMT records. Materially participating trading amounts stay outside that limit.
Prior passive-loss allocations and separately stated trader interest remain
unfinished. A limited investment-tax worksheet is shown when its inputs resolve.
The earned-income-credit investment test separately excludes nonpassive K-1
business amounts, including nonpassive trading income, while retaining allowed
passive losses.
The regular excess-business-loss worksheet includes ordinary K-1 amounts after
basis, at-risk and passive limits, independently of QBI eligibility. For current
losses from ordinary nonpassive businesses with the same QBI percentage, the
disallowed qualified portion is excluded from current QBI and shown separately
in the NOL carryforward. This includes a single business and combinations with
current business profits having the same QBI percentage after current SSTB
eligibility. The SSTB percentage applies once to current income, allocated wages
and property basis. For ordinary nonpassive businesses with only losses,
current and released historical losses can have different QBI percentages.
Earlier limits release historical losses in FIFO order; the excess-loss limit
then preserves the qualified proportion of the released amounts. Mixed
percentages involving positive business income and investment-tax components
still need further allocation and are reported explicitly.
Separate AMT records now support the ordinary-business depreciation adjustment
and independent stock/debt basis, at-risk and passive-loss balances. Missing
AMT records remain unresolved; regular balances are never assumed to apply.
When the necessary worksheet inputs resolve, review notes also show separate
regular and AMT current operating losses, excess-business-loss carryforwards,
debt reductions and total loss carryforwards. Losses still suspended by basis,
at-risk or passive limits are excluded. These supported ordinary-business cases
do not establish full S-corporation, partnership or trust/estate K-1 coverage.

This example demonstrates [workflow interactions](../reference/protocols.md#workflow-interactions),
[client form caching](../reference/annotations.md#cacheidentity), and
[persistent data](../reference/protocols.md#persistence-failure-and-the-no-status-api).

[Build an App](../guides/developing-apps.md) · [Back to Libertas](../index.md)


Basic dependent-filer estimates apply the limited standard deduction and distinguish
eligibility to be claimed from an actual claim for education credits and student-loan
interest. Dependent filers with their own potential dependents or Marketplace
allocations still require additional worksheets. Form 8615 estimates
use the selected parent's source return and other children's worksheet amounts,
allocate the parent-rate increase and retain the child's own-tax minimum.
Parent qualified dividends and net capital gains, and other children's allocated
preferential amounts, retain their preferential rates in that calculation.
The filing child's qualified dividends and net capital gains now receive the
Line 5 allocation automatically, using the standard deduction or supported
personal itemized expenses such as medical costs and home-mortgage interest.
Parent, sibling and filing-child 25%/28% gains use the coordinated Schedule D
worksheets, including tax on the child's remaining income and its own-tax minimum.
Sibling entries preserve original net capital gain and special-gain sources;
on-screen results show both original and allocated amounts for family coordination.
Personal taxes use an income-or-sales election and separate personal-use property
taxes, excluding amounts already deducted in a business or rental. Real-estate taxes on property held solely for taxable investment
appreciation have a separate entry, outside the personal SALT cap, with AMT,
net-investment-income-tax and directly connected Form 8615 deduction treatment.
Portfolio investment-interest entries include current eligible borrowing costs,
separate regular-tax and AMT carryforwards, and eligible private-activity-bond
interest. The App calculates the ordinary Form 4952 income limit, deduction and
next-year disallowed amount, with NIIT and child-tax effects. Loan-use records
can identify interest connected with ordinary income, preferential income, or
both, including prior regular-tax carryforwards. Form 8615 uses that attribution
when it is established by the allowed deduction and source facts.
Qualified dividends and net long-term gains do not raise that limit without an
election. You can choose no election, a capital-gains-first amount, or explicit
capital-gain and dividend amounts. Elected income loses preferential rates;
the App coordinates regular tax and AMT while retaining the original gains for
the QBI limit. The App displays the election but does not file it. Revocation
generally requires IRS consent. Investment gains are tracked separately from personal/home gains;
worthless investment securities, liquidation losses, and both ordinary and capital
portions of qualifying section 1244 stock losses and ordinary section 1242 SBIC
stock losses are included. Worthless
nonbusiness loans enter this pool when their source facts confirm an investment
purpose; wholly personal loans remain outside it. Prior
capital losses enter the limit when source records confirm that the remaining
losses came entirely from investment property. Form 8615 family election
coordination, unresolved loss attribution or loan purpose, exceptional rental
classification, income-tax expense allocation that changes the interest limit,
and unresolved child-tax interest attribution still need worksheets. A binding
interest limit on mixed expenses remains unresolved when the source facts do
not determine which expenses were deducted.
Income-tax payments can coexist with investment interest when every possible
allowed tax allocation gives the same interest deduction and carryforward;
NIIT still uses its separately documented allocation.
The current calculation follows the published [Form 4952](https://www.irs.gov/pub/irs-pdf/f4952.pdf)
and [AMT instructions](https://www.irs.gov/instructions/i6251); final 2026 form
reconciliation remains pending.

Do not include taxes capitalized into basis or deducted elsewhere. Foreign
exclusions, other directly connected expenses and some capital-dependent
business cases still need worksheets. State/local income-tax allocation for NIIT uses the current return's gross
investment-income/AGI ratio or documented source-income amounts. The App applies
the personal SALT cap proportionally. NIIT then caps total eligible investment
itemized deductions at total itemized deductions allowed after section 68;
it does not proportionally reduce each investment expense. Form 4952 runs
before section 68, so that later reduction does not create interest carryforward;
AMT uses its separate deduction without the section 68 reduction. For NIIT,
sales and personal-use property taxes do not reduce investment income. An
unresolved source prevents an affected NIIT calculation from being treated as complete. Personal-service businesses and independent young filers with any combination
of service-only, mixed and capital-only businesses participate in the calculation.
Dependents with mixed or capital-only businesses are also supported when
nonbusiness earnings, reduced by the full half-SE deduction, independently
establish the maximum standard deduction. Other dependent business cases
still need coordination.
Service-only profits and losses are combined; mixed activities use their own
compensation amounts and the shared per-business self-employment-tax deduction
allocation for regular kiddie tax. Dependent filers with capital-only or mixed
businesses still need standard-deduction coordination. These remain rate-schedule
estimates pending final 2026 tax-table reconciliation. See the
[IRS Form 8615 instructions](https://www.irs.gov/instructions/i8615).

The minimum-tax credit worksheet accepts retained 2025 return facts and prior
credit carryforward. It recalculates tax on permanent exclusion items using
2025 exemptions and rates; only the remaining AMT generates credit. A negative
result reduces older credit. Current use follows other nonrefundable credits
and cannot reduce tax below the applicable tentative minimum tax. Unused credit
is shown for the next return, never refunded. Prior qualified dividends and
capital gains use retained 2025 rate-worksheet facts, including the 25% pool and
the separate regular-tax amounts occupying capital-gain bands. The App applies
2025 rates and clips gains to exclusion-only AMT taxable income. Required
worksheets that were never completed or are unresolved must be reviewed;
missing amounts are not assumed to be zero. Foreign-tax or NOL coordination,
ownership changes, legacy vehicle-credit history and debt-related credit
reductions still require additional worksheets. See
[Form 8801](https://www.irs.gov/instructions/i8801) and the
[2025 AMT source form](https://www.irs.gov/pub/irs-pdf/f6251.pdf).
Final 2026 form reconciliation remains required.

For 2026 AMT, young filers use the ordinary filing-status exemption and phaseout.
Section 55(d)(4)(A)(iii) disables the child earned-income cap, and P.L. 119-21
section 70107 removed its sunset. Regular kiddie tax still applies separately.
Rev. Proc. 2025-32 section 4.11 nevertheless lists a child cap; the App follows
the statute and flags this guidance conflict in affected results pending final
IRS reconciliation. See the [current statute](https://uscode.house.gov/view.xhtml?req=granuleid%3AUSC-prelim-title26-section55&num=0&edition=prelim)
and [IRS revenue procedure](https://www.irs.gov/irb/2025-45_IRB).



Ordinary non-W-2 scholarships are calculated from award terms and qualified
spending. Qualifying graduate/postdoctoral non-tuition stipends also provide IRA
compensation. Service awards outside the three named program exceptions, other special exclusions and cross-benefit allocation
elections still need additional worksheets.


Personal nonservice prizes include cash and noncash fair market value, with payer
withholding. Refused awards and qualifying direct-to-charity achievement awards
are excluded. Employment, business, gambling and special Olympic/Paralympic
award treatment use separate rules.


Personal recourse debt cancellation uses the actual discharge date and this
filer's allocated amount, including nondeductible personal interest. Taxable
cancellations enter Schedule 1 and AGI; genuine gifts or bequests are excluded
without tax-attribute reduction. A creditor form and its underlying discharge
are counted once. These amounts are not employment or investment income.
For one insolvency event per filer, the App calculates the exclusion from
before-discharge assets and liabilities. For a nonjoint filer with no other
unresolved loss or credit attributes, it reduces current-year NOL first, then
unused historical losses in original-year order. Regular and AMT losses use
separate calculations, after current tax has been determined. A loss in its
final eligible year is reduced before expiration. The remaining exclusion
reduces personal-use property basis under the Form 982 limit, using separately
entered regular and AMT basis. On-screen results preserve both loss balances
and the basis reductions for future returns. The same worksheet handles
confirmed Chapter 12/13 personal-debt discharges without an insolvency limit.
See [Form 982 instructions](https://www.irs.gov/instructions/i982) and the
[general AMT rule](https://www.law.cornell.edu/cfr/text/26/1.55-1).

Chapter 7/11 separate-estate treatment, joint-owner NOL allocation, excess
business losses, general credit/capital-loss ordering, separate discharge
events, other student-loan relief, deductible debt, property dispositions and
business debt remain unfinished. Retained business assets require a further
property-category worksheet if either regular or AMT losses leave debt to
reduce basis. Unresolved cases produce review issues. See
[IRS Publication 4681](https://www.irs.gov/publications/p4681) and the
[Bankruptcy Tax Guide](https://www.irs.gov/publications/p908).

Ordinary hobby goods sales use receipts and calculated inventory cost, with no
operating-expense deduction or deductible loss. These nonservice property
receipts affect IRA income limits but do not create contribution compensation;
qualifying wages or other compensation can still support an IRA contribution.
Service activities and collectible sales use separate treatment. See
[IRS IRA compensation rules](https://www.irs.gov/publications/p590a).


Capital-loss carryovers use taxable income before the enhanced senior
deduction under section 151. The age/blindness addition to the standard
deduction remains deductible in that calculation. For a prior return, enter
the filed Schedule 1-A line 37 amount separately from total deductions; the App
calculates the carryover. Regular and AMT carryovers are reported separately,
since the same capital losses can be used differently under the two income
calculations. Supply separate retained AMT Schedule D and carryover worksheet
facts when the incoming AMT balance differs, including an AMT-only loss. The
App refigures AMT netting, preferential and special-rate gains, investment
interest and ATNOL use. Its capital-first investment-income election is capped
by AMT income. An explicit gain/dividend allocation is preserved when both
AMT sources cover the chosen amounts, even if regular and AMT capital pools
differ. An allocation exceeding available AMT capital gains still requires
further coordination. See [section 1212(b)(2)](https://www.law.cornell.edu/uscode/text/26/1212)
and [Form 6251 instructions](https://www.irs.gov/instructions/i6251).

Capital-asset sales distinguish investment from personal use. Personal losses
are disallowed per item before gain netting; sale withholding is included once.
For a current sale with a different retained AMT basis or wash-sale adjustment,
supply those source amounts separately. The App calculates both gains and losses,
then applies each system's netting and capital-loss limit. It also refigures AMT
collectible gains, investment-interest limits, ATNOL use and carryovers. Personal
losses remain disallowed per item in both systems. Omit the AMT facts only when
the regular amounts also apply. An established different AMT holding period
can also be supplied; the App nets short- and long-term amounts separately for
each tax system. Unresolved basis or stock-compensation income
still requires review. See [Form 6251's disposition instructions](https://www.irs.gov/instructions/i6251).

Incentive-stock-option sales derive compensation and regular/AMT basis from
Form 3921 prices, grant/exercise dates and the shares sold. Fractional quantities
accept up to six decimal places, normalized to integer millionths before lot
amounts are rounded to cents. Both holding periods
are checked, and each lot's basis, compensation and gains appear on screen.
Enter any amount paid for the option itself separately from its exercise price,
allocated only to the entered shares. This cost reduces the exercise spread and
is included once in stock basis; it is not interest or a second commission.
Confirm whether calculated compensation is already in the same filer's W-2
wages; it is counted once. Otherwise it enters Schedule 1 line 8k as earned
employee compensation, without self-employment or investment-income tax.
Selling fees reduce capital proceeds but not the compensation limit. For a
wash-sale disposition, the full exercise spread can be compensation even when
the usual gain limit would be smaller; split partly affected lots.

A same-year exercise and sale has no ISO AMT adjustment. A later disqualifying
sale reverses regular compensation for AMT and separately applies AMT capital
netting and loss limits. Sold shares belong in the sale entry, not the year-end
held-share exercise list. That list also accepts fractional quantities through
six decimal places and uses integer monetary arithmetic for the AMT spread,
including any option purchase cost allocated to those held shares. Choose
recognition on exercise, when rights first become transferable or cease being
subject to substantial risk of forfeiture, or under a valid timely AMT election.
The recognition event must fall in 2026; an earlier exercise can qualify when
restrictions end in 2026. Use fair market value at that event. The App records
an election already made; it does not file one. Forfeitures, other special histories and prior
AMT-credit carryovers still need further worksheets. For monthly dependent-care
earned-income limits, non-W-2 ISO compensation is assigned automatically to the
sale month before comparison with student/self-care deemed earnings. Reported
W-2 compensation stays in the supplied monthly payroll. Neither allocation
adds income a second time. Young filers' ISO compensation reversals use the
same AMT exemption rule described above. See [IRS stock-option guidance](https://www.irs.gov/publications/p525),
[Form 6251 instructions](https://www.irs.gov/instructions/i6251), and
[Treasury regulation §1.422-1](https://www.law.cornell.edu/cfr/text/26/1.422-1).

Collectible sales apply the maximum 28% regular-tax rate and coordinated AMT
calculation after loss netting. Foreign-income exclusions now recompute the
collectible and unrecaptured-gain amounts separately for regular tax and AMT.
Unlisted property formerly used in business still requires separate worksheets.


Form 1099-DIV box 2d collectible distributions use the same loss-netting and
rate calculation as collectible sales; the amount remains part of box 2a.

Form 2439 entries for individual taxable holdings now calculate ordinary and
collectible undistributed gains, the tax-paid payment credit, and a per-holding
share-basis adjustment. Payer-reported section 1250 components use the shared regular-tax and AMT
calculations. Section 1202 components and nominee or passthrough allocations
still require further work.

Nondividend distributions reduce each share lot's basis before producing taxable
capital gain. The result shows the remaining basis to retain for later payments
and sales. Cash and property liquidations recognize gain as basis is exhausted and a loss
only on final cancellation. Received property gets a separate basis record;
liabilities and special allocations still need further work.

Ordinary main-home sales calculate gain from closing records and evaluate the
full exclusion using ownership, residence and prior-sale facts for each filer.
Reduced exclusions use separate filer periods and listed qualifying events,
including medical-care moves and employment-related inability to meet basic
household expenses.
Depreciation and nonqualified-use allocations are calculated before exclusion.
Other facts-and-circumstances exceptions, separate business portions and special
home-sale adjustments remain unfinished.

Surviving-spouse home sales evaluate the higher exclusion using sale/death dates,
remarriage and the required ownership, residence and prior-sale histories.
Estate returns and inheritance-basis calculations remain outside this path.

Qualifying residence in a licensed care facility can supplement the home-sale
residence test, with separate checks for prior home residence and ownership.

Confirmed wholly worthless nonbusiness cash loans produce short-term capital
losses from unrecovered principal, with ordinary capital-loss limits and
carryforwards. Purchased debts, guarantees and later recoveries need further work.

Wholly worthless investment securities use a deemed year-end sale and the
applicable purchased or inherited holding period, with ordinary capital-loss
netting and carryforwards. Qualifying directly owned Section 1244 stock also supports the shared ordinary-loss allowance, with excess capital loss,
NOL and net-investment-income-tax coordination. The basis calculation handles
later cash contributions and a qualifying tax-free exchange of one investment
property, including ordinary assumed liabilities and its original holding period.
Investment-stock losses do not enlarge Form 461 operating-business losses;
their business-loss treatment applies to the NOL worksheet. Other ownership/basis
cases and mixed holding-period allocation above the section 1244 allowance
remain unfinished.

Stock in an SBA-licensed small business investment company has a separate
**Section 1242** entry. It accepts adjusted basis, acquisition date, sale proceeds
or established worthlessness, and license/ownership confirmation. The App reports
the full ordinary loss without using the section 1244 annual allowance, including
regular and AMT NOL, investment-interest and NIIT effects. Purchases from another
shareholder can qualify. Keep the supporting issuer, share and transaction records
for the required return statement. Trading-business, ownership and basis exceptions
remain outside this entry. See [Treasury regulation 1.1242-1](https://www.govinfo.gov/content/pkg/CFR-2025-title26-vol13/pdf/CFR-2025-title26-vol13-sec1-1242-1.pdf).

State/local income-tax refunds use prior-return deduction records to determine
the taxable recovery. The ordinary path accounts for the alternative sales-tax
deduction, SALT limits and itemization limits;
complex prior-year tax-benefit recoveries remain unfinished.

Social Security and equivalent railroad box 5 amounts may be negative. The calculation combines the
return's amounts, including both spouses on a joint return, before figuring
taxable benefits. Net repayments do not create negative current benefit income.
The $3,000 review threshold combines these net repayments with entered earlier-year
unemployment repayments; larger totals need prior-year deduction-versus-credit
analysis. No deduction or credit is claimed for supported totals at or below $3,000.

For earlier-year Social Security lump sums, the user can retain regular taxation
or compare the election using prior-return and payment records. The calculation
uses the lower taxable amount and preserves total benefits on the current return.
Prior returns are not amended; revoking an election after filing requires IRS
consent. Uncovered traditional IRA contributions and Roth-only contributions
are supported. Traditional contributions with workplace-plan coverage and
changed filing-unit ownership still require additional coordination.

Ordinary nonbusiness copyright, patent and licensing royalties use gross
receipts and documented administration/collection expenses. Net amounts flow
through Schedule E, investment-tax and EIC rules without creating self-employment
earnings or QBI. Mineral depletion, amortization, business services and special
ownership or disposition cases still need separate calculations.

For-profit personal-property rentals outside a business use the same compact
property-income entry. Gross rent and eligible paid expenses appear separately
on Schedule 1; their net amount enters AGI, the NIIT calculation and the combined
royalty/rental EIC test. For Form 4952, ordinary passive rent is excluded. The
App uses total and depreciable unadjusted basis to include net rent when less
than 30% is depreciable, under [IRS recharacterization rules](https://www.irs.gov/publications/p925).
Ordinary passive personal-property profit can release otherwise suspended
real-estate rental losses and is excluded from the special-allowance MAGI; the
profit is reported only once. Recharacterized nonpassive rent supplies no passive
loss capacity. Current personal-property rental losses can absorb other passive profits;
expenses allowed on Schedule 1 and suspended balances are shown separately.
These losses receive no real-estate special allowance. Separate AMT balances
reflect differences in other rental activities. Retained regular-tax and AMT worksheets can also supply prior suspended
operating expenses and depreciation for the same continuing activity. The App releases them
against eligible profits and shows opening expenses, allowed expenses and
closing balances for each activity, including zero balances. Recharacterized
investment income is calculated after the activity’s own prior losses.
Purchased nonbusiness rental equipment uses the shared bonus/MACRS engine,
including qualifying continuing assets and table or manual computation. Enter
purchase and service dates, cost, recovery class, elections and retained asset
history; the App calculates depreciation. These purchases share the return-wide
convention test and class elections with business equipment. Nonbusiness rentals
cannot claim section 179, but do not prevent a separate business from claiming it.
Asset basis decreases even when passive limits suspend the depreciation deduction;
releasing that prior deduction never reduces basis twice. Paid operating expenses,
current depreciation, allowed deductions and suspended balances remain distinct.
See [IRS depreciation guidance](https://www.irs.gov/publications/p946) and
[rental reporting guidance](https://www.irs.gov/taxtopics/tc414).

Continuing equipment acquired before September 28, 2017 is also supported when
placed in service after 1998 using 150% declining balance or straight-line GDS,
with identical regular-tax and AMT basis records. This includes the ordinary
15- and 20-year methods. The original convention and table/manual approach remain
in effect; an earlier purchase date does not create a new bonus deduction.

For earlier-acquired post-1998 equipment requiring regular 200% versus AMT 150%
depreciation, enter the separately retained AMT cumulative depreciation and
original table/manual approach. The App calculates both current deductions and
remaining bases. Rental differences pass through passive limits and investment
interest; business differences adjust AMT income. Regular QBI and self-employment tax remain unchanged. See
[Form 6251 instructions](https://www.irs.gov/instructions/i6251).

Listed property and vehicles, ADS, personal conversions, pre-1999 service,
capital losses, and changed ownership/activity still require further calculations.
Current-year alternative-tax NOLs are calculated separately, including the AMT
nonbusiness-deduction limit and excess-business-loss carryover. The on-screen
result shows current loss and total carryforward for each tax system; an AMT loss
can exist even when the regular NOL is zero. Retained prior-year ATNOL records now supply the AMT deduction, with the ordinary
90% limit, documented special-loss exceptions, oldest-year-first use and
expiration. Ordinary losses cannot consume the capacity reserved for special
losses, even when the ordinary loss arose first. Each loss year shows its opening balance, use, expiration and
remaining carryforward. Loss consumption uses modified AMT income with net
capital losses and QBI added back; it can exceed the current deduction.
AGI-based deductions continue to use regular-tax AGI. Supported foreign employee
pay can include its earned-income exclusion: excluded pay does not increase
ATNOL capacity, and remaining income keeps the AMT foreign tax-rate calculation.
This requires reconciled U.S.-source records for the same taxpayer or unchanged
joint filing unit. Mixed special/ordinary AMT components within a
year, changed ownership, Form 1116 foreign allocation and
general debt-discharge attribute reduction remain unfinished. An eligible
foreign-tax-credit election based on payee statements remains available with
ATNOLs; it uses the same credit for regular tax and AMT. See [IRS ATNOL instructions](https://www.irs.gov/instructions/i6251).

Prior regular NOL records now support standard or itemized
deductions with wages,
interest, dividends, unemployment, operating businesses, supported capital sales,
worthless securities, capital distributions, supported state-tax refunds and bad debts, pension/IRA distributions, Roth conversions, Social Security and supported rentals, including IRA
contributions, student-loan interest and the
non-itemizer cash-charity deduction. Retirement income keeps its distribution
character and basis allocation; an NOL does not erase early-distribution tax.
Medical exceptions use final AGI after the NOL deduction. Own-account Roth
withdrawals preserve contribution/conversion ordering and additional tax on
taxable earnings or recent conversions even when an NOL makes AGI negative;
qualified withdrawals do not enter the NOL income limit. Supported inherited
traditional and Roth IRA distributions also participate: basis recovery, Roth
qualification and the death exception remain independent of the NOL. An
itemized section 691(c) estate-tax deduction still reduces taxable income,
while its consumed estate value remains separate from NOL carryforwards.
See [IRS inherited-IRA guidance](https://www.irs.gov/publications/p590b).
Supported Archer and Medicare MSA withdrawals and inherited HSA/MSA values
also enter the NOL calculation after their medical exclusions. Withdrawal
taxes and Medicare MSA balance relief remain separate from the NOL deduction;
see [IRS health-account guidance](https://www.irs.gov/publications/p969).
Supported long-term-care benefits and ABLE distributions use their taxable
amounts after source-specific exclusions; ABLE additional tax remains payable
even with negative AGI. Existing ABLE investment-interest and investment-tax
limitations still apply. See [Form 8853](https://www.irs.gov/instructions/i8853)
and [Publication 907](https://www.irs.gov/publications/p907). Taxable Social
Security is refigured on the final return and separately for each loss year’s
modified-income calculation; gross benefits remain on the benefit line. The App applies the
post-2017 80% limit and uses eligible loss years oldest first. Each year shows
its opening balance, absorption, expiration and remaining balance. Eligible
seniors keep the additional standard deduction and enhanced senior deduction.
The latter reduces the NOL deduction-limit income, but is excluded when
measuring loss consumption for carryforward. Eligible employee tips, overtime
and vehicle-interest deductions enter both NOL income bases, with phaseouts
refigured after earlier losses. Rental modified AGI retains the NOL deduction;
the active-participation allowance is refigured for both the final return and
each later loss vintage. Suspended passive losses remain separate from NOL
carryforwards. See [Form 8582 line 6](https://www.irs.gov/instructions/i8582).
Employer adoption-benefit exclusions are also refigured after NOL deductions,
including their effect on later-vintage loss absorption and supported special-needs
wage adjustments. See [Form 8839 instructions](https://www.irs.gov/instructions/i8839). A pre-2018
loss can produce a deduction larger than the loss actually absorbed; the App
preserves that difference for later years rather than generating a new loss.
It refigures IRA and student-loan deductions with earlier losses before measuring
how much of each later loss is absorbed. For that measurement it removes the
net capital-loss deduction and refigures dependent deductions; the actual return
and capital-loss carryforward still use the actual capital result. Worthless
securities retain their holding-period category; capital distributions recover
basis before recognizing gain, and liquidation losses wait for finality.
If the NOL prevents capital-loss absorption, that loss remains available in its
separate carryforward. An NOL can also restore Roth contribution
eligibility when the income limit otherwise disallows it.
Supported prizes, scholarships and jury pay also participate, retaining award
exclusions, earned-income classification and the adjustment for jury pay
surrendered to an employer. Disability pensions retain wage/EIC treatment;
qualifying alimony retains IRA compensation but is not EIC earnings, and
recapture adjustments precede the NOL limit. Casual gambling retains its
2026 loss limit and winnings cap; automatic deduction selection compares
complete NOL results for standard and itemized routes. Unused gambling losses
do not become a new NOL. Home-sale gain enters after the section 121 exclusion;
hobby goods retain cost recovery without creating business losses. Qualifying
health-service loan assistance stays excluded, and program-paid student-loan
interest stays nondeductible when the NOL reduces the income phaseout.
State-tax refunds retain the prior-return tax-benefit limit and remain excluded
from AMT income after applying a regular NOL. Nonbusiness royalties and
personal-property rentals participate without treating portfolio income as
passive income. Rental suspensions remain separate for regular tax and AMT;
mixed rental pools refigure the real-estate allowance for each NOL year.
Section 1244 and SBIC losses retain their ordinary-loss rules and separate
capital carryforwards. A new stock-loss NOL is recorded separately from an
unused older operating loss. Supported Form 2439 gains retain their special-rate
components, refundable fund-tax credit and share-basis increase. Taxable
personal debt cancellation enters income; qualifying gifts, student discharges
and seller price adjustments keep their existing exceptions. Supported bankruptcy and insolvency exclusions reduce current and prior
regular/AMT NOL before personal-property basis, as described above;
current tax and the NOL deduction already used remain unchanged. Prior capital returns can now
retain optional loss-year balances from supporting worksheets. Supplied totals
must reconcile to the calculated short- and long-term carryovers; missing
history remains unknown. This does not yet enable general Form 982 reductions.
Regular and AMT balances stay separate, and ordinary operating-loss deductions
do not reduce self-employment earnings or investment income. They do reduce
AGI and can reduce the QBI income limit. Medical and state/local-tax limits
are refigured, and automatic deduction selection compares complete return
calculations. The NOL worksheets exclude QBI before applying the high-income
itemized-deduction limit. Owner SEP contributions follow the
nonbusiness-deduction limit for both regular and AMT current NOLs.
See [Form 172 instructions](https://www.irs.gov/instructions/i172).

This regular-loss path requires reconciled same-filer U.S.-source operating-loss
records without investment or excess-business-loss components. Supported foreign
employee pay now participates, with or without its earned-income exclusion.
Excluded pay does not consume the NOL; remaining taxable income retains foreign
tax-rate stacking, and NIIT modified income adds back the exclusion. Foreign-origin
loss allocation, young/final-return allocation and Form 1116 still need coordinated
worksheets; the App reports these combinations as unsupported.
Current-year public-charity cash gifts now show the itemized deduction, the
percentage-limit excess carried forward, and any floor-disallowed amount that
cannot carry. The 0.5% floor is applied after the 60% ceiling. Its disallowed
amount joins the carryforward only when a percentage-limit excess exists that
year. With prior regular NOLs, the App reduces the charitable carryforward by
the amount the charitable excess increased the surviving NOL carryforward;
the same deduction cannot remain in both balances. Retain 2026 balances for the
five following years. Prior-year charity imports and mixed gift categories
still need further worksheets.
See [Public Law 119-21, section 70425](https://www.govinfo.gov/content/pkg/PLAW-119publ21/html/PLAW-119publ21.htm).

For non-itemizers, a gift larger than the current deduction limit does not reject
the return. The App applies the cash contribution-base limit and the $1,000
($2,000 joint) cap to the deduction, without a 0.5% floor or charitable carryover.
Other recorded calculation gaps remain open.

Social Security-equivalent railroad benefits (RRB-1099) share the SSA-1099
entry and calculation, including signed repayments and the lump-sum election.
Enter withholding from SSA-1099 box 6 or RRB-1099 box 10. Non-equivalent railroad
pensions on RRB-1099-R use the Retirement entry and its Railroad calculation
choice; do not enter them as Social Security. See [IRS Publication 915](https://www.irs.gov/publications/p915).

For an ordinary employer-plan distribution before annuity payments begin, the
App calculates tax-free basis recovery from the payment, nonforfeitable balance
and unrecovered after-tax investment. All three must refer to the same account
or separate employee-contribution contract. Results show remaining basis;
public-safety insurance exclusions apply after recovery. A rollover can transfer
the pretax portion alone, or include after-tax amounts deposited into a traditional
IRA by year-end. With that owner's IRA basis record, the App adds incoming basis
automatically and coordinates withdrawals, conversions and QCDs. Do not add it
again to the prior-basis input. Direct transfers to a qualified employee plan or
403(b) can include after-tax amounts when separate taxable/nontaxable accounting
is confirmed; the output identifies basis transferred to that plan. An eligible
employer-plan cash rollover completed in 2027 reports its after-tax portion
separately as 2027 IRA basis; it does not reduce taxable 2026 IRA withdrawals
or enter the outstanding IRA-to-IRA rollover balance. Legacy contribution-first rules, employer-stock
NUA rollover/recovery allocations and special lump-sum tax elections require further calculations. See [IRS qualified-plan distribution rules](https://www.irs.gov/publications/p575).

For nonemployer investment stock distributed and fully sold in 2026, the App
can allocate retained sale proceeds between pension income and capital gain or
loss while excluding proceeds rolled into an eligible account. Enter the sale
and rollover once in the retirement entry. For example, stock distributed at
$50,000, sold for $40,000, with $25,000 rolled over produces $18,750 pension
income and a $3,750 capital loss before the annual capital-loss limit. Rolling
over all proceeds recognizes neither gain nor loss. This path requires no
after-tax contributions, NUA, wash sale or other special stock treatment.
See [IRS property-rollover examples](https://www.irs.gov/publications/p575).

For an employer-securities distribution, use the payer-reported taxable amount
or the NUA calculation with plan-allocated contributions and appreciation.
The calculation distinguishes a qualifying entire-balance lump sum from other
distributions, which defer only eligible employee-contribution appreciation.
It also supports choosing current taxation instead of lump-sum NUA deferral.
Results show taxable pension income, deferred NUA, recovered contributions and
securities basis. The App checks event timing from the first payment date and the participant’s
birth, death, separation or disability date, together with the latest pre-2026
payment from the same-kind plans. Include actual loan offsets in that history.
An earlier-year payment after the selected event prevents using that event;
a later qualifying event can establish eligibility again. [An IRS ruling illustrates
these distinctions](https://www.irs.gov/pub/irs-wd/0634059.pdf), but private rulings
are not precedent. Same-kind plan completeness and
recovery allocations must be known;
unresolved allocation needs further support. A cash-only rollover is supported
when you retain all employer securities, timely designate the rollover entirely
to cash and separately designate recovered contributions between cash and securities,
and roll over no more than the distributed cash. After-tax cash rolled into a
traditional IRA by year-end automatically enters its required basis worksheet;
a direct employer-plan transfer requires separate taxable/after-tax accounting.
After-tax IRA deposits completed in 2027 use the separate next-year basis
output described above. A payer-reported direct rollover with zero taxable income uses the existing
retirement entry; enter separate Forms 1099-R separately when a direct rollover
accompanies retained NUA stock. Do not combine or repeat the transferred
amount in the retained-stock entry. Calculating a partial securities rollover
or rollover of stock-sale proceeds, and missing-designation ratable allocation,
needs further support.
Report any later stock
sale separately from the distribution. The NUA sale input takes
the sold lot's original distribution value, deferred NUA and net proceeds. It
separates deferred appreciation from subsequent gain or loss and shows stock
basis and both gain components. For example, a lot distributed at $100,000 with
$75,000 deferred NUA has $25,000 basis. A $120,000 sale produces $75,000 long-term
NUA gain and $20,000 additional gain with the actual post-distribution holding
period. Only the latter enters net investment income before other applicable
netting. For shares inherited after the plan distributed them, enter the date
of death and the estate's applicable value for the sold lot: date-of-death
value or the executor's valid elected alternate valuation. The App preserves
deferred NUA, adjusts the remaining basis and treats the sale as long-term.
This currently requires no allocable federal estate or generation-skipping tax;
answering Yes or Not sure marks the result unsupported. An alternate valuation
uses the estate administrator’s applicable value, including an earlier estate
distribution or disposition; the App does not make the estate election.
Estate values below deferred NUA, gifts and other basis changes need further
support. See [the reproduced inherited-stock IRS ruling](https://bradfordtaxinstitute.com/Endnotes/Rev_Rul_75-125.pdf).
See the [IRS NUA sale rules](https://www.irs.gov/publications/p575) and
[NIIT instructions](https://www.irs.gov/instructions/i8960).

Qualified pensions can calculate their taxable amount with the IRS Simplified
Method when the payer has not supplied it. New annuities use original cost,
contract duration and months paid; continuing annuities retain the prior
worksheet's monthly exclusion and recovered cost. Results show current basis
recovery and remaining cost for next year. Ordinary railroad pensions received
at age 60 or later can recover contributory basis while keeping vested-dual and
supplemental benefits fully taxable. Nonqualified annuities outside the fixed-period or lifetime calculation, railroad repayments
or disability treatment, and shared-recipient allocations still need additional work.
See [IRS pension guidance](https://www.irs.gov/publications/p575).

Nonqualified commercial annuities have a separate category for payer-determined
taxable amounts, partial pre-annuity withdrawals or fixed-period/lifetime General Rule calculations. Their taxable income enters the net investment income tax
calculation; qualified retirement distributions do not. Ordinary payments and
early payments without an exception, death/permanent-disability exceptions, and immediate-annuity exceptions
are supported. Those exceptions remove the early-distribution tax without
removing taxable annuity income or its NIIT treatment. Exchanges and other
early-distribution exceptions still need additional work. See the
[IRS early-distribution exceptions](https://www.irs.gov/publications/p575).
Immediate-annuity eligibility uses the original purchase date, annuity starting
date, single-premium contract and substantially equal payments at least annually.
The starting date must be within one year of purchase; a tax-free exchange does
not reset the original purchase date. The starting date must also agree with
any entered General Rule basis calculation.
See [IRS Form 8960 annuity instructions](https://www.irs.gov/instructions/i8960).

For a partial withdrawal before annuity payments begin, enter cash value before
surrender charges and unrecovered investment immediately before that withdrawal.
List the balances of every other annuity required to be combined with the
paying contract. The App combines their values and investment before allocating
the withdrawal to earnings, then shows taxable income, recovered investment
and the group's remaining basis. Do not enter the paying contract twice.
Each later withdrawal needs updated insurer records for the whole group.
The ordinary [section 72(e)(12) grouping rule](https://www.govinfo.gov/content/pkg/USCODE-2024-title26/html/USCODE-2024-title26-subtitleA-chap1-subchapB-partII-sec72.htm)
combines contracts entered into after October 21, 1988, issued by the same
insurer or affiliates to the same policyholder in one calendar year.
The original and recipient contracts in a qualifying partial exchange under
Revenue Procedure 2011-38 section 4.01 remain separate under section 4.03,
even with the same insurer. Do not combine them merely because of the exchange;
use each contract's allocated investment and subsequent insurer records.
Full surrender, exchanges, pre-August 14, 1982 investment and unresolved
grouping need separate calculations; they cannot use this choice.
See [IRS nonperiodic distribution rules](https://www.irs.gov/publications/p575).

For payments that stop at death **or** at the end of a maximum term, choose
**One life, ending no later than a fixed term**. Enter the original nearest-
birthday age and the contract's maximum term in months. The App selects the
Table VIII factor; do not use a lifetime-only factor or treat the term as
payments guaranteed after death. The current worksheet covers ages 5–115 and
terms of 13–480 months using the unisex table for the entire investment.
The IRS example—$200 monthly, age 65, five years maximum—has expected return
of $11,760. A final-annuitant deduction requires death before the term expires.
See [IRS temporary-life annuity rules](https://www.irs.gov/publications/p939).

For a fixed-payment General Rule annuity, count the monthly installments
represented by amounts received this year, including delayed installments from
earlier years. Twelve current installments plus three delayed installments
means **15**, even if they arrived in one transfer. Enter a delayed fractional
original first-period payment separately. Keep the original monthly amount and
exclusion ratio; do not count previously received payments or their recovered
cost again. Remaining investment still caps recovery for post-1986 contracts.
For example, an original $100 monthly payment with a 45% exclusion produces
$675 of recovery for 15 installments, unless remaining investment is lower.
See [IRS part-year and delayed-payment rules](https://www.irs.gov/publications/p939).

For a General Rule joint-and-survivor annuity with payments to both recipients
in the year of the first death, use a separate income entry for each recipient.
On the survivor's worksheet, also enter the **original-payment amount** paid to
the first annuitant earlier that year, excluding cost-of-living increases. The
App calculates that earlier recovery before limiting the survivor's exclusion.
Do not add the first annuitant's receipts to the survivor's gross income or
repeat their current-year recovery in recovery through 2025. For example, if
$2,000 of investment remained and the first annuitant recovered $1,551, the
survivor can recover at most $449. Retain the same common contract records on
both returns. See [IRS joint-annuitant exclusion rules](https://www.irs.gov/publications/p939).

For a variable commercial annuity, enter original investment, payment frequency
and scheduled installment count, plus either original expected-payment facts or
the retained annual exclusion and prior recovery. Include zero-dollar scheduled
installments. A continuing contract requires a full year's schedule except on
the owner's final return: enter only installments belonging to the decedent
through the death date already entered under filer details. Keep beneficiary
and estate payments separate. The App prorates the annual exclusion, caps it by
receipts and remaining investment, and displays recovery and remaining cost.
For example, six monthly installments with a $1,200 annual exclusion allow up
to $600 of recovery; $300 of actual receipts limits recovery to $300. Any final-
annuitant deduction requires the separate linked deduction and its eligibility
facts. If death preceded every installment belonging to the owner, enter zero
installments and zero gross receipts; current-year recovery is zero. A scheduled
installment paying zero dollars still counts. Use the first scheduled period's
start date even if that period would have begun after death. Delayed or
simultaneous split-recipient receipts need further treatment.

Variable payments that stop at death or term expiration use **One life ending
no later than a fixed term** in the expected-installment facts. Enter the original
nearest-birthday age and original months for a new contract. For a supported
shortfall election, enter current age and months remaining from the current
payment period. Table VIII uses no ordinary-lifetime timing adjustment. The App
handles annual, semiannual, quarterly and monthly schedules, limits payments to
the known term, and distinguishes death from term expiration for the separate
final deduction. A remaining term below six months for a new shortfall election
needs further treatment.

For a continuing finite-term contract, retain the expiration date even when no
shortfall election is made. Choose whether payments continue for a fixed period
regardless of death, or end at death if earlier. Enter the first date outside the
contractual term; a new payment period cannot start on or after that date. The
App uses only the ending year's scheduled installments and retains the annual
allowance. If a new election also supplies remaining-term facts, its date and
contract kind must agree. Do not create an election just to enter an ending date.
See [IRS temporary-life and variable-annuity rules](https://www.irs.gov/publications/p939).

For a joint-life variable annuity whose benefit units and annual exclusion stay
unchanged after the first death, use separate income entries for each recipient.
On the survivor's entry, provide the first annuitant's earlier installment count
and gross receipts, and choose the death exception with the first death date.
The App computes earlier recovery from those facts, limits the survivor to the
remaining investment, and displays both amounts separately. Zero-dollar earlier
installments count, but do not recover investment. Keep the same original cost,
annual allowance and recovery through 2025 on both records. A new shortfall
election during this transition or changed benefit units needs further treatment.
See [IRS variable-annuity survivor rules](https://www.irs.gov/publications/p575). See [IRS variable-annuity rules](https://www.irs.gov/publications/p939).

For an outright gift of an entire annuity, enter surrender value as gross and
choose the gift calculation. Supply remaining investment, issue date, transfer
date and any qualifying spouse/divorce treatment. Covered nonspousal transfers
recognize positive earnings and increase the recipient's investment by the
same amount; qualifying section 1041 transfers carry investment without income.
A loss does not become deductible. The App displays recipient investment and
includes any taxable deemed distribution in income, NIIT and applicable early
tax. This covers individual-to-individual gifts before annuitization, not gift-tax
forms, sales, partial interests, trusts, charities or debt. Nonspousal contracts
issued on or before April 22, 1987 need further analysis. See
[IRS annuity transfer rules](https://www.irs.gov/publications/p575).

For a direct annuity-to-annuity exchange, enter transferred cash value as gross
and choose the exchange calculation. Supply original cash surrender value and
remaining investment. Without cash consideration, the App shows transferred
and retained investment without recognizing gain or loss, even when investment
exceeds value. For a full exchange that also returns cash or other property, include both in
gross and enter them separately in the exchange facts. Value other property at
exchange-date fair market value, which becomes its basis. Recognized gain is
limited to realized gain or cash plus property value, whichever is less.
Replacement annuity investment subtracts cash and property value and adds
recognized gain; a loss remains deferred. Ordinary income, NIIT
and applicable early-distribution tax use the recognized amount. Confirm
unchanged obligees and direct insurer transfer. Partial exchanges
also require the completed 180-day payment review described in the input;
this can extend into the following year. An incomplete or failed review remains
unresolved, rather than automatically taxable. Consideration linked to a partial exchange, debt discharge, assumed liabilities, legacy
cash allocations and changed ownership require further calculations. Keep original issue-date
and investment records; an existing recipient contract may have additional
basis. See [IRS exchange basis allocation](https://www.irs.gov/irb/2003-33_IRB)
and [partial-exchange safe harbor](https://www.irs.gov/pub/irs-drop/rp-11-38.pdf).
The [section 1031(b)–(d) rules](https://uscode.house.gov/view.xhtml?req=%28title%3A26+section%3A1031%29)
expressly cover section 1035 exchanges with money or other property.

For a commercial-annuity loan, assignment or pledge treated as a distribution,
select that option and enter the new loan amount or newly assigned/pledged
value as the gross amount. The App calculates income using the same earnings-first rule, then
increases investment by the taxable portion. The untaxed portion does not
recover basis, and later loan repayments do not increase basis again. Results
show the basis increase and the remaining contract or group investment.
These deemed distributions cannot be rolled over. Legacy contracts still need
separate treatment. See [IRS loan and investment rules](https://www.irs.gov/publications/p575).

For a pre-August 14, 1982 annuity, choose the legacy withdrawal calculation
and enter the insurer's separate current value and remaining investment for
old and later contributions. It allocates a partial cash withdrawal in this
order: old investment, old earnings, new earnings, new investment. Both earnings
portions are taxable and enter NIIT; the old-investment earnings are exempt
from early-distribution tax. Results show each component and remaining basis.
This path requires complete insurer allocation with no loss within either
portion. Legacy loans, pledges, exchanges and unresolved allocations remain
unsupported. See [IRS legacy annuity rules](https://www.irs.gov/publications/p575).

A complete cash surrender, redemption or maturity has its own choice. The App
subtracts remaining investment from the final payment, preserving ordinary-income
and early-tax treatment of any gain. It shows an unrecovered surrender loss
without deducting it or carrying it forward. Eligible employer-plan rollovers
transfer only the after-tax cash actually rolled over. This is separate from
the final-annuitant death deduction. See [IRS contract discharge and loss rules](https://www.irs.gov/publications/p575)
and [Public Law 119-21, section 70110](https://www.govinfo.gov/content/pkg/PLAW-119publ21/html/PLAW-119publ21.htm).

For a fixed-period commercial annuity, the App derives expected return from the
original installment amount, payment frequency and contract length, then calculates the tax-free
portion using original cost and prior recovery. It handles a partial first
payment and caps cumulative recovery at cost for starts after 1986. Supported
July–December 1986 starts keep their exclusion even after cost is recovered.
Qualified pensions that began before November 19, 1996 and retained the General
Rule use the same calculation, without adding their income to NIIT. Later qualified fixed-term or single-life guaranteed-installment pensions also qualify when the original starting age
is at least 75 and at least five years of payments are guaranteed. Eligible
public-safety insurance exclusions apply afterward. Earlier starts, other later
qualified-plan contracts, other refund features and special ownership need further
calculations; variable contracts use their separate method. See
[IRS General Rule guidance](https://www.irs.gov/publications/p939).

For a single-life annuity that ends at death, expected return uses the IRS
Table V factor for the age at the birthday nearest the original starting date.
Annual, semiannual, quarterly and monthly payments are supported; lifetime
factors include the original first-payment timing adjustment. The same
cost-recovery calculation then applies. Continuing contracts keep that
original factor. Contribution history or an election must establish that the
newer tables apply to the entire cost. Ages outside the published table and
allocations to older tables require further treatment.

For a single-life contract with guaranteed installments after death, enter the
original guaranteed dollar total. It can include a fractional final installment.
The App calculates the Table VII refund adjustment,
using the lesser of net cost or guaranteed return and rounding the adjustment
to whole dollars. It reduces the investment used for the exclusion ratio, while
recovery limits retain net cost. For example, the IRS age-65 contract costing
$21,053 with $100 monthly payments guaranteed for 17 years has a $2,856
adjustment. The deduction for unrecovered cost cannot be claimed while
installments must continue after death. Cash refunds, fractional original first installments,
beneficiary receipts and guarantees beyond forty years need
further treatment. See [IRS refund-feature guidance](https://www.irs.gov/publications/p939).

Joint-and-survivor contracts use both original ages and the contract's primary
and survivor payments, including reduced survivor payments. The survivor keeps
the original exclusion ratio and shares the remaining unrecovered cost. A year
with payments to both recipients uses the earlier-payment allocation described
above. Joint-life refund guarantees and other special death-year adjustments
still need further treatment.


A qualifying original seller's purchase-price reduction lowers the basis of
retained personal-use property without taxable cancellation income. The App
checks solvency from assets and liabilities and shows the immediate remaining
basis. This path requires confirmed seller-financed purchase principal, no
bankruptcy and continued personal use. A later ordinary 2026 sale can be entered
with the adjustment: the App applies reduced basis, holding period, collectible
treatment, withholding and personal-loss disallowance. Enter that sale only
once. Home sales, business conversions, installment sales and third-party debt
adjustments need further treatment.
See [IRS price-reduction guidance](https://www.irs.gov/publications/p4681).


Student-loan cancellation distinguishes confirmed PSLF/TEPSLF, federal Teacher
Loan Forgiveness and qualifying death/total-permanent-disability discharges.
The death/disability path requires the debt owner's qualifying SSN answer,
including issuance by the return deadline; the number must be on the filed
return. These exceptions do not reduce tax attributes. Ordinary 2026
income-driven repayment forgiveness does not receive the expired broad
2021–2025 exclusion. Other special discharge grounds and combined exclusions still need further treatment. See
[IRS student-loan guidance](https://www.taxpayeradvocate.irs.gov/news/tax-tips/what-to-know-about-student-loan-forgiveness-and-your-taxes/2026/03/)
and [Publication 4681](https://www.irs.gov/publications/p4681).


Qualifying federal Closed School/Borrower Defense discharges and private-loan
unlawful-practice settlements use the separate
[Rev. Proc. 2020-11 safe harbor](https://www.irs.gov/irb/2020-06_IRB#REV-PROC-2020-11).
The App excludes the discharge and explains why it alone does not trigger
recapture of protected prior education credits or deductions. Confirm the
completed relief and qualifying school; an ordinary hardship settlement does
not qualify. Separate refunds still need their own tax treatment.


NHSC and qualifying state health-service loan repayment assistance is excluded
from income. Enter the program-paid interest that is also included in the
student-loan deduction entry; the App removes it before the deduction cap and
phaseout. If the interest entry already excludes that payment, the overlap is
zero. Ordinary employer assistance follows a different rule.
[IRS education guidance](https://www.irs.gov/publications/p970) defines the
qualifying programs and prevents deducting program-paid interest.


Other service-based student-loan cancellations identify the qualifying lender
or refinancing program, eligible school and required service terms. The App
requires service for a qualifying period and profession across a broad employer
class, and screens out prohibited services to the lending school or funding
organization. Confirmed government, qualifying hospital, school and unmet-needs
refinancing programs use the same exclusion.
[IRS service-loan guidance](https://www.irs.gov/publications/p4681) explains
the lender and service requirements.


Loss results show the current-year NOL and disallowed excess-business-loss
carryforward separately, with a combined carryforward total that includes each
once. General debt-related reductions of those attributes still require further
implementation. [Form 172 instructions](https://www.irs.gov/instructions/i172)
explain the carryover worksheet.


Confirmed nonbusiness royalties contribute to the portfolio investment-interest
limit after directly deductible royalty expenses. Those expenses also reduce
the AMT limit. Interest on the royalty activity itself needs separate Schedule E
treatment and is not entered as portfolio borrowing. See
[Form 4952 instructions](https://www.irs.gov/pub/irs-pdf/f4952.pdf).


### Foreign employee housing

For an eligible employee, enter housing expenses once per person and record
each foreign housing location with its inclusive stay dates. The App applies the published 2026 limit and
base amount before calculating the earned-income exclusion. Housing can also be
elected on its own. The result shows each claimant's eligible expenses, location limit, base amount
and housing exclusion separately, while the
combined exclusions feed taxable income and tax stacking. Ordinary employee pay
is the funding source; an employer-designated housing allowance is not required.
Enter U.S. Government or similar tax-free housing allowances paid for you or your
spouse during the claim period separately. The App subtracts them after the
expense limit and base amount, then applies the employer-earnings cap; they do
not become taxable foreign wages. For example, $50,000 expenses at the 2026
general limit produce $18,606 before allowances. A $5,000 tax-free allowance
reduces that amount to $13,606. This follows
[Publication 54's government-allowance rule](https://www.irs.gov/publications/p54).


The location list includes all 109 rows in [Notice 2026-25](https://www.irs.gov/irb/2026-17_IRB)
and the general limit. Full-year calculations use annual limits; partial-year
calculations use the published daily column. Eligible expenses and election
conditions follow [Form 2555](https://www.irs.gov/instructions/i2555).
Moves combine the limits for each location’s qualifying days; overlapping stays
and dates outside the qualifying period are rejected. Repeated stays at the same
location are combined before applying its limit.
Combined employee/business housing funding, second households and additional
related-deduction coordination remain outside this employee path. Separate
self-employed housing claims are described below. Final 2026 form reconciliation and any later-authorized option to use
2027 limits remain pending.


On a joint return, spouses can leave the household claim with one spouse or
allocate shared expenses between two separate claims. Enter the same total
household expenses on both entries and each claimant's chosen share; the App
checks that the shares do not exceed the total. Each claimant uses their own
qualifying period and full base amount. The App preserves this choice rather
than automatically choosing a tax-minimizing allocation. Two separate households
are supported when both spouses confirm the tax-home and household commuting
distance conditions in [Form 2555's married-couple rules](https://www.irs.gov/instructions/i2555).
On a joint return, both claims must be entered. On married-filing-separately
returns, each spouse enters only their own qualifying household and confirms
the conditions involving both tax homes; the other spouse's full return is not required.


For married filing separately with a shared household, use the separate-return
allocation choice. Enter total household expenses and the spouse's agreed expense
share from their records, not their calculated exclusion. The App checks those
expenses against your share and applies your own period, base and income limit.
Confirm that the allocation is reconciled and contains no double-counted costs.


## Foreign service businesses

Enter an ordinary business once, choosing its income source by where services
were performed, not the payer's address. The wholly foreign service choice
supports ordinary unexcluded income and an optional earned-income exclusion for
the qualifying-business cases described below, subject to its stated U.S. SE-tax
and ordinary-treatment conditions. Receipts and expenses
remain in the business calculation; the result identifies foreign net income.
The App excludes that income and its related deductions from QBI while retaining
self-employment tax. This follows the IRS rules for
[foreign self-employment](https://www.irs.gov/individuals/international-taxpayers/self-employment-tax-for-businesses-abroad)
and [qualified business income](https://www.irs.gov/instructions/i8995).

Domestic and wholly foreign service businesses can coexist. Ordinary owner
health deductions and the existing eligible owner-only SEP path remain available
when that business does not claim the exclusion.
Mixed-source businesses, foreign property/vehicle/home-office treatment, foreign
business credit allocation, additional exclusion coordination and excess-loss attribution
when mixed-character losses coexist with profitable businesses still need calculation support; unresolved cases do not publish
a completed estimate. This is calculation support in the prototype, not a
filing-readiness claim.


When the business loss is entirely from supported foreign service activities,
the App applies the annual excess-business-loss limit and reports the resulting
NOL carryforward. It separately identifies the suspended amount as nonqualified
for QBI, without adding it a second time to the carryforward. Current allowed
foreign losses likewise do not create a QBI loss carryforward. Mixed domestic/foreign returns are also supported when losses occur on only
one side: foreign losses stay nonqualified despite domestic profit, and ordinary
fully eligible domestic losses stay qualified despite foreign profit. The result separates
suspended qualified loss from the current QBI loss carryforward. Losses on both
sides are supported for ordinary domestic businesses when every business
has zero income or a loss: the App applies any specified-service eligibility
percentage for the loss year, allocates the suspended amount between qualified
and nonqualified losses, and reports both. The nonqualified amount can include
ineligible domestic service-business loss as well as foreign loss. Mixed-character losses with profitable
businesses, and special entity/property combinations, still need allocation support.


On a joint return, the other spouse may independently claim the supported
employee earned-income and housing exclusions. The App tracks the election by
person while applying the joint return's tax-stacking and credit rules. Foreign business income and employee pay for the same person can share an
exclusion when their confirmed election and qualifying period agree and that
person has no housing claim.


For foreign service businesses, including loss activities, select the earned-income exclusion
and enter the qualifying residence or presence facts. All foreign service businesses owned by
the same person must use the same confirmed election and qualifying period;
the App combines their receipts and expenses under one exclusion limit. All entered
receipts must belong to that qualifying period. The App uses gross receipts for
the prorated exclusion limit, then calculates the business expenses and half-SE
deduction allocable to excluded income. The result shows gross exclusion,
disallowed deductions and the net Schedule 1 exclusion. SE tax remains based on
full profit; excluded profit cannot absorb another business's loss.

This path permits an independent employee or business exclusion for the other
spouse. Same-person foreign employee pay shares the business exclusion cap and
ratio. It currently requires no housing claim for that person
and no owner health or retirement-plan deduction for the excluded business. Those combinations, a single business with mixed domestic/foreign operations and
combined employee/self-employed housing funding still require coordinated calculation support.
The method follows [Form 2555 lines 20 and 44–45](https://www.irs.gov/instructions/i2555),
using the published 2026 exclusion limit; final 2026 form reconciliation remains
pending.


When wages and service-business receipts belong to the same claimant, the App
reports each source's share of the one gross exclusion. Only business expenses
and attributable half-SE deductions enter the business-related deduction
adjustment. The business's net excluded amount adjusts the regular and AMT
business-loss limit; the employee share does not. A combined housing exclusion
or deduction remains a separate unfinished calculation.


When related expenses exceed excluded receipts, the net exclusion is negative:
it restores expenses to income. For example, $20,000 of fully excluded receipts
and $30,000 of expenses produce a negative $10,000 net exclusion, leaving no
business loss to offset other income. With $200,000 receipts, $300,000 expenses
and the full $132,900 exclusion cap, $199,350 of expenses are disallowed and
$33,550 of loss remains. These adjustments do not create IRA compensation or
U.S. QBI. Regular tax and AMT use zero, rather than a negative amount, for tax-rate
stacking. See [Publication 54, deductions related to excluded income](https://www.irs.gov/publications/p54).


A claimant can also have separately entered U.S. businesses. Their receipts stay
outside the foreign exclusion. The App allocates the deductible half of SE tax
once between the businesses, uses only the foreign share for Form 2555, and
retains the U.S. share in QBI. For example, $50,000 profit in each source produces
$14,130 SE tax. With the U.S. record receiving the rounding dollar, its half-SE
share is $3,533 and the foreign share is $3,532; the net foreign exclusion is
$46,468 and AGI is $46,467. This case still requires no owner health or retirement
plan deduction for that person. Mixed operations within one business require
additional source allocation.


A foreign service-business owner with no foreign employee pay can enter a housing
claim on one business entry. It uses the same location, dates, expenses, allowance
and household rules as employee housing. A housing deduction can be elected with
or without FEIE. All applicable claims must use a consistent qualifying period;
do not repeat the household claim for each business.

The App limits the current housing deduction to foreign earned income remaining
after gross exclusions. If a 2025 amount remains, enter the retained housing
amount before its income limit and the amount deducted that year. Current-year
housing uses the available income first; any unused 2025 balance expires after
2026. The result separately shows the current deduction, prior balance used,
expired prior balance and current amount carried into 2027.

The housing deduction appears in income adjustments, separately from the
foreign exclusion. It reduces neither SE tax nor IRA compensation. For example,
$200,000 foreign service receipts, $50,000 business expenses and $50,000 housing
expenses at the general location limit produce a $18,606 housing deduction,
$28,164 AGI and $24,090 total tax, including unchanged $21,194 SE tax. This follows
[Form 2555 Part IX](https://www.irs.gov/instructions/i2555) and
[Publication 54's IRA compensation rule](https://www.irs.gov/publications/p54).

Prior self-employed housing carryovers can also use remaining foreign employee
income. Combined employee/business funding now allocates one housing claim between
exclusion and deduction. Creating a new carryover without foreign earnings
remains unsupported. A valid prior carryover can be used with no current housing
amount, subject to available foreign income; with no income capacity it expires. For NIIT, the separate MAGI
calculation adds back FEIE less related disallowed deductions; it does not add
back the housing exclusion or deduction, following the
[Form 8960 MAGI worksheet](https://www.irs.gov/instructions/i8960).

For a second foreign household maintained for a spouse or dependents because
conditions at the taxpayer's tax home are dangerous, unhealthful or otherwise
adverse, select the corresponding household choice. Confirm eligibility and
include its eligible expenses in the housing total without duplicating claims
or adding tax-free employer lodging. The current calculation supports
general-limit locations with one expense cap and one base amount; high-cost
or differing-location combinations remain unsupported. This follows the
[Form 2555 second-household instructions](https://www.irs.gov/instructions/i2555).

Ordinary sales of nonstatutory-option shares or vested stock awards can use the
capital-sale input when compensation was correctly reported as wages and the
adjusted basis and holding period are established. Use the complete adjusted
basis, which may differ from the broker's reported basis; do not tax the same
compensation twice or add it twice to basis. This route does not calculate
unresolved compensation from grant, vesting or exercise facts, or handle
special deferral elections or unresolved restricted-property histories. See
[IRS Topic 427](https://www.irs.gov/taxtopics/tc427).

The ordinary employee stock-sale worksheet also accepts acquisition and sale
records directly: amount paid, fair market value when unrestricted shares were
acquired, acquisition fees and sale proceeds. It derives compensation, basis
and holding period for ordinary nonstatutory-option shares and vested awards
whose compensation was fully reported as wages in the acquisition year. The
compensation is displayed but not taxed again. Enter one sold lot and do not
repeat it as another capital sale. Valid, timely section 83(b) elections use transfer-date value and the original
transfer date; later vesting does not reset basis or tax compensation again.
This covers ordinary sales after vesting with complete wage reporting. An
explicit forfeiture option also handles still-unvested stock under an effective
election: only the amount paid less the amount returned can create a capital
loss; previously taxed compensation is not deducted. This path currently requires
no transaction fees and no payment exceeding the amount paid. Revoked elections
and section 83(i) deferrals remain unsupported. See
[IRS Revenue Procedure 2012-29](https://www.irs.gov/irb/2012-28_IRB).
ESPP dispositions use the separate worksheet below.

For an ordinary employee stock purchase plan sale, the section 423 worksheet
uses Form 3922 and sale records to calculate compensation, stock basis and
capital gain. It distinguishes qualifying and early sales and avoids adding
compensation already in W-2 wages twice. Otherwise, compensation enters
Schedule 1 line 8k. Share quantities accept up to six decimal places and are normalized to
integer millionths before computing lot amounts, rounded to cents. Current
support requires shares acquired by the employee on exercise;
a later transfer to a broker does not restart the holding period. Use the fixed
grant price when Form 3922 box 8 is blank for a fixed-price option. Special
dispositions require further worksheets. See [Publication 525](https://www.irs.gov/publications/p525).

ISO and section 423 worksheets also recognize the statutory holding-period
exception for an eligible person's sale under a qualifying federal certificate
of divestiture. This does not turn a short-term capital gain into a long-term
gain, and ordinary employer conflict policies do not qualify. A separate
section 1043 gain-deferral election remains unsupported. See
[26 U.S.C. 421(d)](https://www.govinfo.gov/content/pkg/USCODE-2024-title26/html/USCODE-2024-title26-subtitleA-chap1-subchapD-partII-sec421.htm).

ISO sales can include a restriction history when shares vested before sale.
The statutory qualification test still uses the transfer date, while regular
compensation on a disqualifying sale uses vesting value and its capital holding
period starts at vesting. AMT follows its own recognition date and value; a
valid AMT election does not fix regular compensation at exercise. A recognition
event in the sale year is included once, without repeating sold shares in the
year-end held-share list. See
[the final ISO regulations, example 2](https://www.irs.gov/irb/2004-36_IRB).
Restricted-ISO forfeitures and sales before vesting remain separate cases.


Historical repayment comparisons can include ordinary railroad pension income
from the original RRB-1099-R and its Simplified Method worksheet. Enter original
gross and contributory benefits separately; tax-free employee-cost recovery
cannot reduce vested dual benefits or supplemental annuities. Historical dates,
months and prior recovery belong to the selected original year. Enter current-year box 8 repayments separately, identifying original year and
whether the repaid benefit was contributory or from boxes 5/6. Do not net these
against current pension income or repeat same-year repayments already netted in
payment boxes. Fully taxable prior benefits can use the shared deduction/credit
comparison, including the combined $3,000 threshold with unemployment repayments.
The original year, recipient and benefit component must reconcile; historical
worksheets currently cover 2023–2025 within their stated limits. Repayments
involving tax-free employee-cost recovery or unknown years need further analysis.


Confirmed permanent-and-total railroad disability payments taxable as wages
before minimum retirement age use the disability-pension entry. Enter only the
non-Social-Security-equivalent portion documented for that treatment; RRB-1099
SSEB benefits use the Social Security entry. Do not infer the split from age
alone or include either amount twice. Occupational disability alone, unresolved
basis and payments straddling retirement-age treatment need further analysis.
See [RRB component rules](https://rrb.gov/Benefits/TXB-85/SectionB) and
[IRS disability-pension reporting](https://www.irs.gov/publications/p575).


A younger railroad survivor can use the Railroad pension entry with the death
exception and documented death/payment dates. The payment must follow death;
survivor pension income stays separate from both disability wages and SSEB.
Any employee-cost recovery must use the survivor's correctly allocated records.
The App does not determine eligibility for an RRB benefit award. Shared
Simplified Method recovery requires the common records described below. See [RRB survivor benefits](https://rrb.gov/Benefits/Survivor/GeneralInfoSurvivor/Articles/TypesofSurvivorBenefits).


For an ordinary railroad spouse or divorced-spouse annuity, leave the employee
cost worksheet absent: these benefits are fully taxable and do not share the
employee's exclusion. This differs from a spouse's own employee annuity or a
survivor annuity. On a joint return, enter each recipient's statement separately;
do not duplicate employee contributions. See
[RRB employee-contribution rules](https://www.rrb.gov/Benefits/TXB-85/SectionD).


Simultaneous pension annuitants can enter the original recipient payment and
total payments that established their share of the monthly exclusion. Use the
whole contract's cost, combined prior recovery and monthly exclusion before
allocation; entering an already allocated amount would reduce it twice.
Results show both monthly amounts and remaining whole-contract cost. For
example, an $840 share of $1,500 total payments receives $84 of a $150 monthly
exclusion. This recipient must receive payments throughout the entered months, with
complete common recovery history. For fractional cents, keep one beneficiary
order in the common allocation record. Enter the sum of original monthly
payments preceding this recipient (zero for the first); use the same record on
every recipient's return. The App allocates rounding cents consistently so the
shares add up to the common exclusion, including its final cost limit. This
ordering is the App's rounding convention, not an IRS ordering requirement.
For a documented entitlement change, such as a child's annuity ending, add the
payment-month number and new share. Month 2 means the second payment month in
this worksheet, not necessarily February. Leave the new share absent when this
recipient becomes the sole remaining annuitant. The App applies changes in
month order and stops recovery when common cost runs out; cost-of-living
increases alone do not establish a new allocation. The final monthly exclusion
is shown for retained records. If other annuitants received payments earlier in the same year, enter those
earlier common payment months separately. The App reduces available cost before
calculating this recipient's exclusion; do not also include those months in this
recipient's count or in prior-year recovery. The common monthly exclusion must
be unchanged. A changed common monthly amount and excess-exclusion carryovers
still need further calculations. Remaining cost is the balance after the entered
months; reconcile any later payments to other annuitants before next year's
worksheet.
See [IRS survivor allocation examples](https://www.irs.gov/publications/p721).


A temporary CSRS/FERS survivor annuity for one nondisabled child, without a
surviving-spouse annuity, uses a separate worksheet. Enter original cost and the
payment period to age 22, prior cost recovered, prior unused exclusion and each
2026 monthly payment in order, including zero payments. The App limits the
monthly exclusion to the payment, carries unused exclusion forward and stops
recovery at the remaining cost. Results show exclusion, unused exclusion and
remaining investment. An unused amount does not itself authorize benefits after
termination or a deduction. If the annuity ends early, enter the plan's last
entitled payment date and include payments only through that month. Keep the
original recovery period; ending early does not accelerate cost recovery.
Any final lump sum must be entered separately. Multiple temporary children,
changed recovery periods need further worksheets. A final-annuitant deduction
uses the separate linked deduction entry described below. See
[IRS temporary-child-annuity rules](https://www.irs.gov/publications/p721).


For a CSRS/FERS child permanently disabled at the annuity starting date, with no
surviving-spouse annuity, use the pension Simplified Method with **One lifetime**
and the child's age at that starting date. If other temporary child beneficiaries
share the annuity, enter their payment shares against this common lifetime
exclusion. Do not combine a temporary sibling's age into a joint-life factor.
Use the death exception for survivor payments; these remain pension income,
not the child's disability wages. See
[IRS disabled-child survivor rules](https://www.irs.gov/publications/p721).


For an ordinary CSRS/FERS regular-contribution death-benefit lump sum, enter the
full gross payment, original employee cost and all earlier tax-free annuity
recovery. The App calculates the taxable excess and identifies any cost left
unrecovered. For example, $38,400 paid plus $1,000 previously recovered against
$45,000 cost produces no taxable lump sum. This worksheet covers the entire
final benefit paid to one individual, with death-exception treatment. Split
beneficiaries, transfers, special lump-sum elections and separately treated
voluntary contributions need their own calculations. A deduction on the
annuitant's final return is separate from this beneficiary's income. See
[IRS civil-service lump-sum worksheets](https://www.irs.gov/publications/p721).


On the final annuitant's own final return, link an unrecovered-annuity-cost
deduction to that person's periodic pension entry. Enter original cost, prior
year recovery and other tax-free payouts. The App subtracts current recovery;
a linked Simplified Method worksheet includes its earlier/common annuitant
recovery. Cost and history must agree between the records. A temporary child's
annuity must end in the death month. The result enters itemized deductions;
it is not a deduction on a beneficiary's death-benefit lump-sum entry. See
[IRS unrecovered-cost guidance](https://www.irs.gov/publications/p575).
