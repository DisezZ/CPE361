#include <stdio.h>
#include <math.h>

void plotElipse(int pointContainer_x[50], int pointContainer_y[50], int index, int Tx, int Ty, int quadrant) {
    int i;
    int x_sign = 1;
    int y_sign = 1;

    if(quadrant == 2) {
        x_sign = -1;
    } else if(quadrant == 3) {
        x_sign = -1;
        y_sign = -1;
    } else if(quadrant == 4) {
        y_sign = -1;
    }

    for(i=0; i<index; i++) {
        printf("%d, %d\n", Tx+(x_sign * pointContainer_x[i]) , Ty+(y_sign * pointContainer_y[i]));
    }
}

void findEllipse(Tx, Ty, Xradius, Yradius) {
    int twoASquare = 2*Xradius*Xradius;
    int twoBSquare = 2*Yradius*Yradius;
    int X = Xradius;
    int Y = 0;
    int XChange = Yradius*Yradius*(1 - (2*Xradius));
    int YChange = Xradius * Xradius;
    int ellipseErr = 0;
    int stoppingX = twoBSquare * Xradius;
    int StoppingY = 0;

    int pointContainer_x[50];
    int pointContainer_y[50];
    int index = 0;

    while( stoppingX >= StoppingY ) {
        printf("%d, %d\n", X, Y);
        pointContainer_x[index] = X;
        pointContainer_y[index] = Y;
        index++;
        Y++;
        StoppingY += twoASquare;
        ellipseErr += YChange;
        YChange += twoASquare;
        if( (2*ellipseErr) + XChange > 0 ) {
            X--;
            stoppingX -= twoBSquare;
            ellipseErr += XChange;
            XChange += twoBSquare;
        }
    }

    X = 0;
    Y = Yradius;
    XChange = Yradius * Yradius;
    YChange = Xradius * Xradius * (1 - (2*Yradius));
    ellipseErr = 0;
    stoppingX = 0;
    StoppingY = twoASquare * Yradius;
    while ( stoppingX <= StoppingY ) {
        printf("%d, %d\n", X, Y);
        pointContainer_x[index] = X;
        pointContainer_y[index] = Y;
        index++;
        X++;
        stoppingX += twoBSquare;
        ellipseErr += XChange;
        XChange += twoBSquare;
        if( (2 * ellipseErr) + YChange > 0 ) {
            Y--;
            StoppingY -= twoASquare;
            ellipseErr += YChange;
            YChange += twoASquare;
        }
    }
    plotElipse(pointContainer_x, pointContainer_y, index, Tx, Ty, 2);
    plotElipse(pointContainer_x, pointContainer_y, index, Tx, Ty, 3);
    plotElipse(pointContainer_x, pointContainer_y, index, Tx, Ty, 4);
}

int main() {
    int x,y,a,b;
    printf("XOrgin yOrigin xRadius yRadius\n");
    scanf("%d %d %d %d", &x, &y, &a, &b);

    findEllipse(0, 0, a, b);
}