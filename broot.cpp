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


  Job j1 = {1, 237, 23, 61};
  igloo.insert(make_pair(1, j1));
  Job j2 = {2, 757, 51, 29};
  igloo.insert(make_pair(2, j2));
  Job j3 = {3, 132, 56, 79};
  igloo.insert(make_pair(3, j3));
  Job j4 = {4, 185, 41, 69};
  igloo.insert(make_pair(4, j4));
  Job j5 = {5, 330, 27, 23};
  igloo.insert(make_pair(5, j5));
  Job j6 = {6, 622, 25, 65};
  igloo.insert(make_pair(6, j6));
  Job j7 = {7, 1212, 12, 55};
  igloo.insert(make_pair(7, j7));
  Job j8 = {8, 305, 14, 38};
  igloo.insert(make_pair(8, j8));
  Job j9 = {9, 1403, 21, 40};
  igloo.insert(make_pair(9, j9));
  Job j10 = {10, 41, 40, 77};
  igloo.insert(make_pair(10, j10));
  Job j11 = {11, 534, 54, 33};
  igloo.insert(make_pair(11, j11));
  Job j12 = {12, 405, 46, 81};
  igloo.insert(make_pair(12, j12));
  Job j13 = {13, 642, 45, 37};
  igloo.insert(make_pair(13, j13));
  Job j14 = {14, 1091, 53, 9};
  igloo.insert(make_pair(14, j14));
  Job j15 = {15, 50, 35, 47};
  igloo.insert(make_pair(15, j15));
  Job j16 = {16, 1299, 36, 82};
  igloo.insert(make_pair(16, j16));
  Job j17 = {17, 1372, 19, 36};
  igloo.insert(make_pair(17, j17));
  Job j18 = {18, 793, 42, 43};
  igloo.insert(make_pair(18, j18));
  Job j19 = {19, 1360, 21, 57};
  igloo.insert(make_pair(19, j19));
  Job j20 = {20, 955, 37, 57};
  igloo.insert(make_pair(20, j20));
  Job j21 = {21, 1332, 7, 7};
  igloo.insert(make_pair(21, j21));
  Job j22 = {22, 1090, 23, 95};
  igloo.insert(make_pair(22, j22));
  Job j23 = {23, 594, 49, 75};
  igloo.insert(make_pair(23, j23));
  Job j24 = {24, 1059, 27, 97};
  igloo.insert(make_pair(24, j24));
  Job j25 = {25, 1330, 58, 5};
  igloo.insert(make_pair(25, j25));
  Job j26 = {26, 1012, 59, 93};
  igloo.insert(make_pair(26, j26));
  Job j27 = {27, 571, 56, 72};
  igloo.insert(make_pair(27, j27));
  Job j28 = {28, 1188, 12, 26};
  igloo.insert(make_pair(28, j28));
  Job j29 = {29, 284, 13, 87};
  igloo.insert(make_pair(29, j29));
  Job j30 = {30, 1122, 45, 72};
  igloo.insert(make_pair(30, j30));
  Job j31 = {31, 537, 21, 62};
  igloo.insert(make_pair(31, j31));
  Job j32 = {32, 878, 9, 55};
  igloo.insert(make_pair(32, j32));
  Job j33 = {33, 523, 1, 1};
  igloo.insert(make_pair(33, j33));
  Job j34 = {34, 780, 6, 14};
  igloo.insert(make_pair(34, j34));
  Job j35 = {35, 76, 8, 45};
  igloo.insert(make_pair(35, j35));
  Job j36 = {36, 82, 25, 69};
  igloo.insert(make_pair(36, j36));
  Job j37 = {37, 1108, 18, 65};
  igloo.insert(make_pair(37, j37));
  Job j38 = {38, 1254, 33, 70};
  igloo.insert(make_pair(38, j38));
  Job j39 = {39, 640, 20, 58};
  igloo.insert(make_pair(39, j39));
  Job j40 = {40, 1190, 37, 64};
  igloo.insert(make_pair(40, j40));
  Job j41 = {41, 480, 57, 47};
  igloo.insert(make_pair(41, j41));
  Job j42 = {42, 1209, 27, 23};
  igloo.insert(make_pair(42, j42));
  Job j43 = {43, 914, 24, 17};
  igloo.insert(make_pair(43, j43));
  Job j44 = {44, 1257, 2, 20};
  igloo.insert(make_pair(44, j44));
  Job j45 = {45, 315, 52, 91};
  igloo.insert(make_pair(45, j45));
  Job j46 = {46, 500, 7, 73};
  igloo.insert(make_pair(46, j46));
  Job j47 = {47, 100, 50, 50};
  igloo.insert(make_pair(47, j47));
  Job j48 = {48, 47, 55, 4};
  igloo.insert(make_pair(48, j48));
  Job j49 = {49, 580, 48, 60};
  igloo.insert(make_pair(49, j49));
  Job j50 = {50, 227, 8, 76};
  igloo.insert(make_pair(50, j50));
  Job j51 = {51, 286, 28, 19};
  igloo.insert(make_pair(51, j51));
  Job j52 = {52, 206, 48, 51};
  igloo.insert(make_pair(52, j52));
  Job j53 = {53, 204, 16, 62};
  igloo.insert(make_pair(53, j53));
  Job j54 = {54, 668, 47, 33};
  igloo.insert(make_pair(54, j54));
  Job j55 = {55, 874, 31, 10};
  igloo.insert(make_pair(55, j55));
  Job j56 = {56, 1439, 44, 11};
  igloo.insert(make_pair(56, j56));
  Job j57 = {57, 400, 6, 65};
  igloo.insert(make_pair(57, j57));
  Job j58 = {58, 353, 45, 94};
  igloo.insert(make_pair(58, j58));
  Job j59 = {59, 511, 32, 30};
  igloo.insert(make_pair(59, j59));
  Job j60 = {60, 67, 2, 55};
  igloo.insert(make_pair(60, j60));
  Job j61 = {61, 401, 23, 97};
  igloo.insert(make_pair(61, j61));
  Job j62 = {62, 1294, 19, 27};
  igloo.insert(make_pair(62, j62));
  Job j63 = {63, 862, 20, 44};
  igloo.insert(make_pair(63, j63));
  Job j64 = {64, 791, 29, 17};
  igloo.insert(make_pair(64, j64));
  Job j65 = {65, 1353, 39, 95};
  igloo.insert(make_pair(65, j65));
  Job j66 = {66, 973, 56, 62};
  igloo.insert(make_pair(66, j66));
  Job j67 = {67, 860, 30, 58};
  igloo.insert(make_pair(67, j67));
  Job j68 = {68, 1349, 57, 1};
  igloo.insert(make_pair(68, j68));
  Job j69 = {69, 659, 15, 95};
  igloo.insert(make_pair(69, j69));
  Job j70 = {70, 770, 48, 99};
  igloo.insert(make_pair(70, j70));
  Job j71 = {71, 1185, 17, 87};
  igloo.insert(make_pair(71, j71));
  Job j72 = {72, 1326, 29, 70};
  igloo.insert(make_pair(72, j72));
  Job j73 = {73, 1006, 47, 67};
  igloo.insert(make_pair(73, j73));
  Job j74 = {74, 1035, 4, 5};
  igloo.insert(make_pair(74, j74));
  Job j75 = {75, 530, 47, 76};
  igloo.insert(make_pair(75, j75));
  Job j76 = {76, 813, 8, 9};
  igloo.insert(make_pair(76, j76));
  Job j77 = {77, 596, 38, 90};
  igloo.insert(make_pair(77, j77));
  Job j78 = {78, 836, 43, 66};
  igloo.insert(make_pair(78, j78));
  Job j79 = {79, 828, 40, 11};
  igloo.insert(make_pair(79, j79));
  Job j80 = {80, 234, 49, 94};
  igloo.insert(make_pair(80, j80));
  Job j81 = {81, 547, 44, 45};
  igloo.insert(make_pair(81, j81));
  Job j82 = {82, 1044, 56, 84};
  igloo.insert(make_pair(82, j82));
  Job j83 = {83, 1012, 27, 40};
  igloo.insert(make_pair(83, j83));
  Job j84 = {84, 84, 37, 24};
  igloo.insert(make_pair(84, j84));
  Job j85 = {85, 209, 46, 31};
  igloo.insert(make_pair(85, j85));
  Job j86 = {86, 973, 58, 57};
  igloo.insert(make_pair(86, j86));
  Job j87 = {87, 1437, 12, 35};
  igloo.insert(make_pair(87, j87));
  Job j88 = {88, 234, 21, 55};
  igloo.insert(make_pair(88, j88));
  Job j89 = {89, 42, 33, 83};
  igloo.insert(make_pair(89, j89));
  Job j90 = {90, 220, 32, 51};
  igloo.insert(make_pair(90, j90));
  Job j91 = {91, 5, 43, 85};
  igloo.insert(make_pair(91, j91));
  Job j92 = {92, 973, 31, 14};
  igloo.insert(make_pair(92, j92));
  Job j93 = {93, 508, 52, 70};
  igloo.insert(make_pair(93, j93));
  Job j94 = {94, 574, 47, 64};
  igloo.insert(make_pair(94, j94));
  Job j95 = {95, 318, 57, 12};
  igloo.insert(make_pair(95, j95));
  Job j96 = {96, 1009, 19, 18};
  igloo.insert(make_pair(96, j96));
  Job j97 = {97, 1240, 2, 25};
  igloo.insert(make_pair(97, j97));
  Job j98 = {98, 288, 6, 7};
  igloo.insert(make_pair(98, j98));
  Job j99 = {99, 628, 14, 98};
  igloo.insert(make_pair(99, j99));
  Job j100 = {100, 98, 19, 48};
  igloo.insert(make_pair(100, j100));


    cout << "The optimal profit is " << findMaxProfit(igloo, 0);
    return 0;
}
