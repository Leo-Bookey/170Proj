// C++ program for weighted job scheduling using Naive Recursive Method
#include <iostream>
#include <algorithm>
#include <unordered_map>
using namespace std;

// A job has start time, duration and profit.
struct Job
{
    int index, finish, duration, profit;
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
        inclProf += findMaxProfit(igloos, time + currActualJob.duration);
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

  unordered_map<int, Job> igloo;


  Job j1 = {1, 2, 2, 138};
  igloo.insert(make_pair(1, j1));
  Job j2 = {2, 2, 2, 583};
  igloo.insert(make_pair(2, j2));
  Job j3 = {3, 2, 2, 868};
  igloo.insert(make_pair(3, j3));
  Job j4 = {4, 2, 2, 822};
  igloo.insert(make_pair(4, j4));
  Job j5 = {5, 2, 2, 783};
  igloo.insert(make_pair(5, j5));
  Job j6 = {6, 2, 2, 65};
  igloo.insert(make_pair(6, j6));
  Job j7 = {7, 2, 2, 262};
  igloo.insert(make_pair(7, j7));
  Job j8 = {8, 2, 2, 121};
  igloo.insert(make_pair(8, j8));
  Job j9 = {9, 2, 2, 508};
  igloo.insert(make_pair(9, j9));
  Job j10 = {10, 2, 2, 780};
  igloo.insert(make_pair(10, j10));
  Job j11 = {11, 2, 2, 461};
  igloo.insert(make_pair(11, j11));
  Job j12 = {12, 2, 2, 484};
  igloo.insert(make_pair(12, j12));
  Job j13 = {13, 2, 2, 668};
  igloo.insert(make_pair(13, j13));
  Job j14 = {14, 2, 2, 389};
  igloo.insert(make_pair(14, j14));
  Job j15 = {15, 2, 2, 808};
  igloo.insert(make_pair(15, j15));
  Job j16 = {16, 2, 2, 215};
  igloo.insert(make_pair(16, j16));
  Job j17 = {17, 2, 2, 97};
  igloo.insert(make_pair(17, j17));
  Job j18 = {18, 2, 2, 500};
  igloo.insert(make_pair(18, j18));
  Job j19 = {19, 2, 2, 30};
  igloo.insert(make_pair(19, j19));
  Job j20 = {20, 2, 2, 915};
  igloo.insert(make_pair(20, j20));


    cout << "The optimal profit is " << findMaxProfit(igloo, 0);
    return 0;
}
