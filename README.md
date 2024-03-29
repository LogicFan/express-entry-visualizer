# Canada Express Entry Visualizer

Canada Express Entry is a point-based immigration program of the Canadian Government, you can read more about it in [How Express Entry Work](https://www.canada.ca/en/immigration-refugees-citizenship/services/immigrate-canada/express-entry/works.html).

This repo contains a website that code of website https://logicfan.github.io/, which visualize statistics and analytical result from recent Express Entry draw.

## Invitation

### Invitation Score
Data are directly from IRCC, with Federal Skill Trade (FST) and Trade Category are merged as same category due to their similarity in items of NOC requirement.

### Invitation Size
Data are directly from IRCC, with Federal Skill Trade (FST) and Trade Category are merged as same category due to their similarity in items of NOC requirement.

## Candidate (a.k.a Pool)

### Count
Data are directly from IRCC

### Rate
The increase rate for each pool bucket if there is no IRCC draw. Here are the assumptions
1. candidate who get their ITA will submit their application uniformly within 15 days window
2. each category follows the same score distribution as the general.
3. diagram is smoothed using exponential moving average with 60 days windows.
4. the projected rate uses the average increase rate of 180 most recent observed days.
