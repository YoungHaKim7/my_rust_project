#include <stdio.h>

int calculate(int bottom, int top)
{
  if (top > bottom)
  {
    int sum = 0;

    for (int number = bottom; number <= top; number++)
      {
        if (number % 2 == 0)
        {
          sum += number;
        }
      } 

    return sum;
  }
  else
  {
    return 0;
  }
}

int main() {
  printf("calculate funtion : (5, 12) %i\n",calculate(5, 12));
  printf("calculate funtion : (5, 3) %i\n",calculate(5, 3));
}
