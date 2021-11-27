// C++ program for weighted job scheduling using Naive Recursive Method
#include <iostream>
#include <algorithm>
#include <unordered_map>
#include <list>
#include <map>
#include <ctime>
#include <cstdio>
using namespace std;

// A job has start time, duration and profit.
struct Job
{
    int index, finish, duration;
    double profit;
};



// A recursive function that returns the maximum possible
// profit from given array of jobs.  The array of jobs must
// be sorted according to finish time.
// timeLeft is the current time (up until 1440 minutes, starts at 0)
long double findMaxProfit(map<int, Job> igloos, int time)
{
    if (time >= 1440 || 0 >= igloos.size()) return 0;
    long double maxProf = 0;
    long double inclProf = 0;
    map<int,Job>::iterator it;
    for (it = igloos.begin(); it != igloos.end(); it++) {
      Job currActualJob = it->second;
      if (currActualJob.profit >= 0) {
        if (currActualJob.duration + time <= 1440) {
          if (currActualJob.duration + time <= currActualJob.finish) {
            inclProf = currActualJob.profit;
          }
          else {
            int late = currActualJob.duration + time - currActualJob.finish;
            inclProf = currActualJob.profit * exp(-0.017 * late);
          }
          it->second.profit *= -1;
          inclProf += findMaxProfit(igloos, time + currActualJob.duration);
          it->second.profit *= -1;
        }
      }
      maxProf = std::max(maxProf,inclProf);
    }
    //cout << maxProf;
    return maxProf;
}

// The main function that returns the maximum possible
// profit from given array of jobs

// Driver program
int main()
{

  map<int, Job> igloo;


  Job j1 = {1, 1, 1, 138};
  igloo.insert(make_pair(1, j1));
  Job j2 = {2, 1, 1, 583};
  igloo.insert(make_pair(2, j2));
  Job j3 = {3, 1, 1, 868};
  igloo.insert(make_pair(3, j3));
  Job j4 = {4, 1, 1, 822};
  igloo.insert(make_pair(4, j4));
  Job j5 = {5, 1, 1, 783};
  igloo.insert(make_pair(5, j5));
  
/*
Job j6 = {6, 1, 1, 65};
  igloo.insert(make_pair(6, j6));
  Job j7 = {7, 1, 1, 262};
  igloo.insert(make_pair(7, j7));
  Job j8 = {8, 1, 1, 121};
  igloo.insert(make_pair(8, j8));
  Job j9 = {9, 1, 1, 508};
  igloo.insert(make_pair(9, j9));
  Job j10 = {10, 1, 1, 780};
  igloo.insert(make_pair(10, j10));


   Job j11 = {11, 1, 1, 461};
  igloo.insert(make_pair(11, j11));
  Job j12 = {12, 1, 1, 484};
  igloo.insert(make_pair(12, j12));
  Job j13 = {13, 1, 1, 668};
  igloo.insert(make_pair(13, j13));
  Job j14 = {14, 1, 1, 389};
  igloo.insert(make_pair(14, j14));
  Job j15 = {15, 1, 1, 808};
  igloo.insert(make_pair(15, j15));
  Job j16 = {16, 1, 1, 215};
  igloo.insert(make_pair(16, j16));
  Job j17 = {17, 1, 1, 97};
  igloo.insert(make_pair(17, j17));
  Job j18 = {18, 1, 1, 500};
  igloo.insert(make_pair(18, j18));
  Job j19 = {19, 1, 1, 30};
  igloo.insert(make_pair(19, j19));
  Job j20 = {20, 1, 1, 915};
  igloo.insert(make_pair(20, j20));

*/

    clock_t start, end;
    start = clock();

    cout << "The optimal profit is " << findMaxProfit(igloo, 0);
    end = clock();
    double timeTaken = double(end - start) / double(CLOCKS_PER_SEC);
    cout << " Time taken is: " << fixed << timeTaken;
    return 0;
}
