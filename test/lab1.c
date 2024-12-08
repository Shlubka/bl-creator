int main(void)
{
  int **mass = (int **)malloc(size_mass_col * sizeof(int *));
  for (int i = 0; i < size_mass_col; i++)
  {
    mass[i] = (int *)malloc(size_mass_row * sizeof(int));
  }

  for (int i = 0; i < size_mass_col; i++)
  {
    for (int j = 0; j < size_mass_row; j++)
    {
      mass[i][j] = rand() % 141 - 40;
      printf("%d ", mass[i][j]);
      if (mass[i][j] > MAX_VAL )
      {
        MAX_VAL = mass[i][j];
      }
      if (mass[i][j] < MIN_VAL)
      {
        MIN_VAL = mass[i][j];
      }
      else {
        printf("goyda");
      }
    }
    printf("\n");
  }
}
