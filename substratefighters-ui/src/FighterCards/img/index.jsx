import React from 'react';

const IMAGES = {
  background: [
    require('./male/background1.png'),
    require('./male/background2.png'),
    require('./male/background3.png'),
    require('./male/background4.png'),
    require('./male/background5.png')
  ],
  clothes: [
    require('./male/clothes1.png'),
    require('./male/clothes2.png'),
    require('./male/clothes3.png'),
    require('./male/clothes4.png'),
    require('./male/clothes5.png'),
    require('./male/clothes6.png'),
    require('./male/clothes7.png'),
    require('./male/clothes8.png'),
    require('./male/clothes9.png'),
    require('./male/clothes10.png'),
    require('./male/clothes11.png'),
    require('./male/clothes12.png'),
    require('./male/clothes13.png'),
    require('./male/clothes14.png'),
    require('./male/clothes15.png')
  ],
  eyes: [
    require('./male/eye1.png'),
    require('./male/eye2.png'),
    require('./male/eye3.png'),
    require('./male/eye4.png'),
    require('./male/eye5.png'),
    require('./male/eye6.png'),
    require('./male/eye7.png'),
    require('./male/eye8.png'),
    require('./male/eye9.png'),
    require('./male/eye10.png')
  ],
  face: [
    require('./male/face1.png'),
    require('./male/face2.png'),
    require('./male/face3.png'),
    require('./male/face4.png')
  ],
  hair: [
    require('./male/hair1.png'),
    require('./male/hair2.png'),
    require('./male/hair3.png'),
    require('./male/hair4.png'),
    require('./male/hair5.png'),
    require('./male/hair6.png'),
    require('./male/hair7.png'),
    require('./male/hair8.png'),
    require('./male/hair9.png'),
    require('./male/hair10.png'),
    require('./male/hair11.png'),
    require('./male/hair12.png'),
    require('./male/hair13.png'),
    require('./male/hair14.png'),
    require('./male/hair15.png'),
    require('./male/hair16.png'),
    require('./male/hair17.png'),
    require('./male/hair18.png')
  ],
  mouth: [
    require('./male/mouth1.png'),
    require('./male/mouth2.png'),
    require('./male/mouth3.png'),
    require('./male/mouth4.png'),
    require('./male/mouth5.png'),
    require('./male/mouth6.png'),
    require('./male/mouth7.png'),
    require('./male/mouth8.png'),
    require('./male/mouth9.png'),
    require('./male/mouth10.png'),
    require('./male/mouth11.png'),
    require('./male/mouth12.png'),
    require('./male/mouth13.png'),
    require('./male/mouth14.png'),
    require('./male/mouth15.png'),
    require('./male/mouth16.png'),
    require('./male/mouth17.png'),
    require('./male/mouth18.png')
  ]
}

const dnaToAttributes = (dna) => {
  let attr = (index, options) => {
    return dna[index] % options;
  }

  return {
    background: IMAGES.background[attr(0, 5)],
    clothes: IMAGES.clothes[attr(1, 15)],
    eyes: IMAGES.eyes[attr(0, 9)],
    face: IMAGES.face[attr(0, 4)],
    hair: IMAGES.hair[attr(4,10)],
    mouth: IMAGES.mouth[attr(0,18)],
  }
}

export const Avatar = (props) => {
  const outerStyle = { height: '150px', position: 'relative', width: '50%' };
  const innerStyle = { height: '150px', position: 'absolute', top: '0%', left: '15px' };

  let avatar = dnaToAttributes(props.dna);

  return (
    <div>
      <div style={outerStyle}>
        <img alt='background' src={avatar.background} style={innerStyle} />
        <img alt='face' src={avatar.face} style={innerStyle} />
        <img alt='hair' src={avatar.hair} style={innerStyle} />
        <img alt='mouth' src={avatar.mouth} style={innerStyle} />
        <img alt='clothes' src={avatar.clothes} style={innerStyle} />
        <img alt='eyes' src={avatar.eyes} style={innerStyle} />
      </div>
    </div>
  )
}
