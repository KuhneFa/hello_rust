#include <stdio.h>

int main() {
	int i = 1;

	if(i == 1){
		i++;
	}
	else if(i == 2){
		return i;
	 }
	printf("i: %d\n", i);
	return i;
}
