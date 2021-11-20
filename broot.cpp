// C++ program for weighted job scheduling using Naive Recursive Method
#include <iostream>
#include <algorithm>
using namespace std;

// A job has start time, duration and profit.
struct Job
{
    int finish, duration, profit;
};



// A recursive function that returns the maximum possible
// profit from given array of jobs.  The array of jobs must
// be sorted according to finish time.
// timeLeft is the current time (up until 1440 minutes, starts at 0)
double findMaxProfit(unordered_map<int, Job> igloos, int time)
{
    // Base case
    if (time >= 1440 || 0 >= igloos.size()) return 0;

    // Find profit when current job is included

    double maxProf = 0;
    double inclProf = 0;
    for (std::pair<int, Job> currJobMap : igloos) {
      Job currActualJob = currJobMap.second;
      int currJobIndex = currJobMap.first;

      if (currActualJob.duration + time <= 1440) {
        if (currActualJob.duration + time <= currActualJob.finish) {
          inclProf = currActualJob.profit;
        }
        else {
          int late = currActualJob.duration + time - currActualJob.finish;
          inclProf = currActualJob.profit * exp(-0.017 * late);
        }
        igloos.erase(currJobIndex);
        inclProf += findMaxProfitRec(igloos, time + currActualJob.duration);
        igloos.insert(make_pair(currJobIndex, currActualJob));
      }
      maxProf = std::max(maxProf,inclProf);


    }
    return maxProf;
}

// The main function that returns the maximum possible
// profit from given array of jobs

// Driver program
int main()
{
    Job arr[] = {{3, 10, 20}, {1, 2, 50}, {6, 19, 100}, {2, 100, 200}};
    int n = sizeof(arr)/sizeof(arr[0]);
    cout << "The optimal profit is " << findMaxProfit(arr, n);
    return 0;
}
