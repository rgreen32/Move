function calculateDisplacement(){
    var timedelta = Date.now() - start;
    var displacement = (block.initialVelocity * (timedelta/1000)) + (.5*-9.8*Math.pow((timedelta/1000), 2))
    return displacement
}

function metersToPixelsDistanceY(height: number){
    let distanceInPixels = canvas.height - (heightRatio * height)
    return distanceInPixels
}

function metersToPixelsDistanceX(distance: number){
    let distanceInPixels = null
    if(distance > 0){
        distanceInPixels = (canvas.width/2) + widthRatio * distance
    }else if(distance < 0){
        let distanceFromOriginPixels = -widthRatio * distance
        distanceInPixels = (canvas.width/2) - distanceFromOriginPixels
    } else{
        distanceInPixels = (canvas.width/2) 
    }
    return distanceInPixels
}