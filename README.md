We set various limits in the DSMS project.

Those limits were defined kind of arbitrarily initially. And as real customer onboarded to AutoML, we quickly see customer hit those limits. E.g., our first customer (hidden digital) was limited by 20 max columns. Our second customer (band lab) created 2 datasets with 199.9 K rows (i.e., hitting the 100K datasets rows limit).

We should have the capability to increase the limit whenever customer asks. While at the same time, we would like to understand how reasonable our current limit is.

The analysis is performed against the datasets on huggingface, which is the largest ML community nowadays.
